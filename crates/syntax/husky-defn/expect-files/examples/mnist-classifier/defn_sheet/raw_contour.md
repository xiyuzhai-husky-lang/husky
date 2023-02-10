Ok(
    DefnSheet {
        defns: [
            Type(
                RegularStruct(
                    RegularStructTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        decl: RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ast_idx: 199,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    32,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    37,
                                                ),
                                                items: ArenaIdxRange(
                                                    2..2,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    38,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
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
                                                    33,
                                                ),
                                                ident: `ConnectedComponent`,
                                                entity_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    39,
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
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 192,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            31,
                                        ),
                                    },
                                    ty: 1,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 160,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            35,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            36,
                                        ),
                                    },
                                    ty: 4,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        34,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        40,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    41,
                                ),
                            },
                        },
                    },
                ),
            ),
            Type(
                Enum(
                    EnumTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        decl: EnumTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            ast_idx: 201,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                    roots: [],
                                },
                            },
                            implicit_parameter_decl_list: None,
                        },
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                            ast_idx: 206,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    412,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    415,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                        ident: `row`,
                                                        token_idx: TokenIdx(
                                                            406,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            410,
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
                                                                value: 225,
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
                                                                value: 174,
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
                                                    ident: `row`,
                                                    access_start: TokenIdx(
                                                        407,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `j`,
                                                    access_start: TokenIdx(
                                                        411,
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
                                        405,
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
                                                407,
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
                                                411,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            409,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        413,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        414,
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
                                        416,
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::i32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            412,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: TypePath(`core::num::i32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            415,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                                ident: `row`,
                                                                token_idx: TokenIdx(
                                                                    406,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `j`,
                                                                token_idx: TokenIdx(
                                                                    410,
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
                                                                        value: 225,
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
                                                                        value: 174,
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
                                                            ident: `row`,
                                                            access_start: TokenIdx(
                                                                407,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `j`,
                                                            access_start: TokenIdx(
                                                                411,
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
                                                    kind: OutputType,
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                421,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                423,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 0,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                422,
                                            ),
                                            ropd: 1,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `row`,
                                            token_idx: TokenIdx(
                                                418,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                420,
                                            ),
                                            item: 2,
                                            rpar_token_idx: TokenIdx(
                                                424,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: PureClosed(
                                                Shr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                419,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                417,
                                            ),
                                            item: 5,
                                            rpar_token_idx: TokenIdx(
                                                425,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                427,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 6,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                426,
                                            ),
                                            ropd: 7,
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
                                            expr_idx: 8,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `row`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `j`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
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
                                        kind: BlockExpr,
                                        expr: 9,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            9,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                            ast_idx: 207,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    433,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    437,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    440,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                        ident: `row`,
                                                        token_idx: TokenIdx(
                                                            431,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            435,
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
                                                                value: 225,
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
                                                                value: 174,
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
                                                    ident: `row`,
                                                    access_start: TokenIdx(
                                                        432,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `j`,
                                                    access_start: TokenIdx(
                                                        436,
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
                                        430,
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
                                                432,
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
                                                436,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            434,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        438,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        439,
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
                                        441,
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::i32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            433,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            437,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: TypePath(`core::num::i32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            440,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                                ident: `row`,
                                                                token_idx: TokenIdx(
                                                                    431,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `j`,
                                                                token_idx: TokenIdx(
                                                                    435,
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
                                                                        value: 225,
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
                                                                        value: 174,
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
                                                            ident: `row`,
                                                            access_start: TokenIdx(
                                                                432,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `j`,
                                                            access_start: TokenIdx(
                                                                436,
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
                                                    kind: OutputType,
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `row`,
                                            token_idx: TokenIdx(
                                                443,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                445,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 0,
                                            opr: PureClosed(
                                                Shr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                444,
                                            ),
                                            ropd: 1,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                442,
                                            ),
                                            item: 2,
                                            rpar_token_idx: TokenIdx(
                                                446,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                448,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                447,
                                            ),
                                            ropd: 4,
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
                                            expr_idx: 5,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `row`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `j`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
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
                                        kind: BlockExpr,
                                        expr: 6,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            6,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                            ast_idx: 208,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    454,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    458,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    461,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                        ident: `row`,
                                                        token_idx: TokenIdx(
                                                            452,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            456,
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
                                                                value: 225,
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
                                                                value: 174,
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
                                                    ident: `row`,
                                                    access_start: TokenIdx(
                                                        453,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `j`,
                                                    access_start: TokenIdx(
                                                        457,
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
                                        451,
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
                                                453,
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
                                                457,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            455,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        459,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        460,
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
                                        462,
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::i32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            454,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            458,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: TypePath(`core::num::i32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            461,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                                ident: `row`,
                                                                token_idx: TokenIdx(
                                                                    452,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `j`,
                                                                token_idx: TokenIdx(
                                                                    456,
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
                                                                        value: 225,
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
                                                                        value: 174,
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
                                                            ident: `row`,
                                                            access_start: TokenIdx(
                                                                453,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `j`,
                                                            access_start: TokenIdx(
                                                                457,
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
                                                    kind: OutputType,
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                467,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                469,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 0,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                468,
                                            ),
                                            ropd: 1,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `row`,
                                            token_idx: TokenIdx(
                                                464,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                466,
                                            ),
                                            item: 2,
                                            rpar_token_idx: TokenIdx(
                                                470,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: PureClosed(
                                                Shr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                465,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                463,
                                            ),
                                            item: 5,
                                            rpar_token_idx: TokenIdx(
                                                471,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                473,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 6,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                472,
                                            ),
                                            ropd: 7,
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
                                            expr_idx: 8,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `row`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `j`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
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
                                        kind: BlockExpr,
                                        expr: 9,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            9,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                            ast_idx: 209,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 3,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    479,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    483,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    487,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    491,
                                                ),
                                                ident: `Direction`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                        ident: `row_above`,
                                                        token_idx: TokenIdx(
                                                            477,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `row_below`,
                                                        token_idx: TokenIdx(
                                                            481,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            485,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                            Parameter,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 229,
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
                                                                value: 230,
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
                                                                value: 174,
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
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: `row_above`,
                                                    access_start: TokenIdx(
                                                        478,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `row_below`,
                                                    access_start: TokenIdx(
                                                        482,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `j`,
                                                    access_start: TokenIdx(
                                                        486,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 2,
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
                                            RegularParameter {
                                                pattern: 2,
                                                ty: 2,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 3,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        476,
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
                                                478,
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
                                                482,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 2,
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                486,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            480,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            484,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            488,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        489,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        490,
                                    ),
                                },
                            ),
                            return_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        492,
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            TypePath(`core::num::i32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 3,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            479,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            483,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            487,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: TypePath(`core::num::i32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            491,
                                                        ),
                                                        ident: `Direction`,
                                                        entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                                ident: `row_above`,
                                                                token_idx: TokenIdx(
                                                                    477,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `row_below`,
                                                                token_idx: TokenIdx(
                                                                    481,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `j`,
                                                                token_idx: TokenIdx(
                                                                    485,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 229,
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
                                                                        value: 230,
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
                                                                        value: 174,
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
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            ident: `row_above`,
                                                            access_start: TokenIdx(
                                                                478,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `row_below`,
                                                            access_start: TokenIdx(
                                                                482,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `j`,
                                                            access_start: TokenIdx(
                                                                486,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 2,
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
                                                    RegularParameter {
                                                        pattern: 2,
                                                        ty: 2,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: OutputType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `row_above`,
                                            token_idx: TokenIdx(
                                                498,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                500,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                497,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..3,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                501,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `row_below`,
                                            token_idx: TokenIdx(
                                                507,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                509,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::FunctionCall {
                                            function: 4,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                506,
                                            ),
                                            arguments: ArenaIdxRange(
                                                5..7,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                510,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                496,
                                            ),
                                            ident: `get_pixel_pair`,
                                            entity_path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                505,
                                            ),
                                            ident: `get_pixel_pair`,
                                            entity_path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    493,
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
                                                        495,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                3,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    502,
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
                                                        504,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                7,
                                            ),
                                        },
                                        Stmt::Match,
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `pixel_pair_above`,
                                                    token_idx: TokenIdx(
                                                        494,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `pixel_pair_below`,
                                                    token_idx: TokenIdx(
                                                        503,
                                                    ),
                                                },
                                                liason: None,
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
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 231,
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
                                                            value: 232,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `row_above`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `row_below`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `j`,
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `pixel_pair_above`,
                                                access_start: TokenIdx(
                                                    495,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            620,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `pixel_pair_below`,
                                                access_start: TokenIdx(
                                                    504,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            620,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
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
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                            ast_idx: 210,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    625,
                                                ),
                                                ident: `Direction`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    629,
                                                ),
                                                ident: `Direction`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    632,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
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
                                                        ident: `inward`,
                                                        token_idx: TokenIdx(
                                                            623,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `outward`,
                                                        token_idx: TokenIdx(
                                                            627,
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
                                                                value: 236,
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
                                                                value: 237,
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
                                                    ident: `inward`,
                                                    access_start: TokenIdx(
                                                        624,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `outward`,
                                                    access_start: TokenIdx(
                                                        628,
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
                                        622,
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
                                                624,
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
                                                628,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            626,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        630,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        631,
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
                                        633,
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            TypePath(`core::num::i32`, `Alien`),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            625,
                                                        ),
                                                        ident: `Direction`,
                                                        entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            629,
                                                        ),
                                                        ident: `Direction`,
                                                        entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            632,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: TypePath(`core::num::i32`, `Alien`),
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
                                                                ident: `inward`,
                                                                token_idx: TokenIdx(
                                                                    623,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `outward`,
                                                                token_idx: TokenIdx(
                                                                    627,
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
                                                                        value: 236,
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
                                                                        value: 237,
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
                                                            ident: `inward`,
                                                            access_start: TokenIdx(
                                                                624,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `outward`,
                                                            access_start: TokenIdx(
                                                                628,
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
                                                    kind: OutputType,
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `outward`,
                                            token_idx: TokenIdx(
                                                640,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                TypePath(`core::num::i32`, `Alien`),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 0,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                641,
                                            ),
                                            ropd: 1,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `inward`,
                                            token_idx: TokenIdx(
                                                646,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                TypePath(`core::num::i32`, `Alien`),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                647,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                639,
                                            ),
                                            item: 2,
                                            rpar_token_idx: TokenIdx(
                                                643,
                                            ),
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                645,
                                            ),
                                            item: 5,
                                            rpar_token_idx: TokenIdx(
                                                649,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 6,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                644,
                                            ),
                                            ropd: 7,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                638,
                                            ),
                                            item: 8,
                                            rpar_token_idx: TokenIdx(
                                                650,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                TypePath(`core::raw_bits::r32`, `Alien`),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 9,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                651,
                                            ),
                                            ropd: 10,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                637,
                                            ),
                                            item: 11,
                                            rpar_token_idx: TokenIdx(
                                                653,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                657,
                                            ),
                                        ),
                                        Expr::MethodCall {
                                            self_expr: 12,
                                            dot_token_idx: TokenIdx(
                                                654,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `last_bits`,
                                                token_idx: TokenIdx(
                                                    655,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                656,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                13..14,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                658,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..2,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                642,
                                            ),
                                            ident: `i32`,
                                            entity_path: TypePath(`core::num::i32`, `Alien`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                648,
                                            ),
                                            ident: `i32`,
                                            entity_path: TypePath(`core::num::i32`, `Alien`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                652,
                                            ),
                                            ident: `r32`,
                                            entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    634,
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
                                                        636,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                14,
                                            ),
                                        },
                                        Stmt::Match,
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `raw_angle_change`,
                                                    token_idx: TokenIdx(
                                                        635,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 238,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `inward`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `outward`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `raw_angle_change`,
                                                access_start: TokenIdx(
                                                    636,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            684,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
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
                                        expr: 15,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            15,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                            ast_idx: 211,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::raw_bits::r32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 3,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 4,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    689,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    693,
                                                ),
                                                ident: `r32`,
                                                entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    697,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    701,
                                                ),
                                                ident: `Direction`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    705,
                                                ),
                                                ident: `Direction`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                        ident: `row_above`,
                                                        token_idx: TokenIdx(
                                                            687,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `row_below`,
                                                        token_idx: TokenIdx(
                                                            691,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            695,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `inward_direction`,
                                                        token_idx: TokenIdx(
                                                            699,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                            Parameter,
                                            Parameter,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 229,
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
                                                                value: 230,
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
                                                                value: 174,
                                                            },
                                                        ),
                                                    ),
                                                    2,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 241,
                                                            },
                                                        ),
                                                    ),
                                                    3,
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
                                                    ident: `row_above`,
                                                    access_start: TokenIdx(
                                                        688,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `row_below`,
                                                    access_start: TokenIdx(
                                                        692,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `j`,
                                                    access_start: TokenIdx(
                                                        696,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `inward_direction`,
                                                    access_start: TokenIdx(
                                                        700,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 3,
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
                                            RegularParameter {
                                                pattern: 2,
                                                ty: 2,
                                            },
                                            RegularParameter {
                                                pattern: 3,
                                                ty: 3,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 4,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        686,
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
                                                688,
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
                                                692,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 2,
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                696,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 3,
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                700,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            690,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            694,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            698,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            702,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        703,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        704,
                                    ),
                                },
                            ),
                            return_ty: Ok(
                                OutputTypeExpr {
                                    expr: 4,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        706,
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            TypePath(`core::num::i32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 3,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 4,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            689,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            693,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: TypePath(`core::raw_bits::r32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            697,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: TypePath(`core::num::i32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            701,
                                                        ),
                                                        ident: `Direction`,
                                                        entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            705,
                                                        ),
                                                        ident: `Direction`,
                                                        entity_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                                ident: `row_above`,
                                                                token_idx: TokenIdx(
                                                                    687,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `row_below`,
                                                                token_idx: TokenIdx(
                                                                    691,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `j`,
                                                                token_idx: TokenIdx(
                                                                    695,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `inward_direction`,
                                                                token_idx: TokenIdx(
                                                                    699,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 229,
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
                                                                        value: 230,
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
                                                                        value: 174,
                                                                    },
                                                                ),
                                                            ),
                                                            2,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 241,
                                                                    },
                                                                ),
                                                            ),
                                                            3,
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
                                                            ident: `row_above`,
                                                            access_start: TokenIdx(
                                                                688,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `row_below`,
                                                            access_start: TokenIdx(
                                                                692,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `j`,
                                                            access_start: TokenIdx(
                                                                696,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `inward_direction`,
                                                            access_start: TokenIdx(
                                                                700,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 3,
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
                                                    RegularParameter {
                                                        pattern: 2,
                                                        ty: 2,
                                                    },
                                                    RegularParameter {
                                                        pattern: 3,
                                                        ty: 3,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: OutputType,
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `row_above`,
                                            token_idx: TokenIdx(
                                                712,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                714,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                711,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..3,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                715,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `row_below`,
                                            token_idx: TokenIdx(
                                                721,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                723,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::FunctionCall {
                                            function: 4,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                720,
                                            ),
                                            arguments: ArenaIdxRange(
                                                5..7,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                724,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                710,
                                            ),
                                            ident: `get_pixel_pair`,
                                            entity_path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                719,
                                            ),
                                            ident: `get_pixel_pair`,
                                            entity_path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    707,
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
                                                        709,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                3,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    716,
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
                                                        718,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                7,
                                            ),
                                        },
                                        Stmt::Match,
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `pixel_pair_above`,
                                                    token_idx: TokenIdx(
                                                        708,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `pixel_pair_below`,
                                                    token_idx: TokenIdx(
                                                        717,
                                                    ),
                                                },
                                                liason: None,
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
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 231,
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
                                                            value: 232,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `row_above`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `row_below`,
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `j`,
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `inward_direction`,
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `pixel_pair_above`,
                                                access_start: TokenIdx(
                                                    709,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            911,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `pixel_pair_below`,
                                                access_start: TokenIdx(
                                                    718,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            911,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
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
            Type(
                RegularStruct(
                    RegularStructTypeDefn {
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        decl: RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                            ast_idx: 212,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    916,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    920,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
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
                                    913,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 243,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            914,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            915,
                                        ),
                                    },
                                    ty: 0,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 244,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            918,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            919,
                                        ),
                                    },
                                    ty: 1,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        917,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        921,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    922,
                                ),
                            },
                        },
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                            ast_idx: 213,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    928,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    929,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            },
                                            Expr::Application {
                                                function: 0,
                                                argument: 1,
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
                                                    930,
                                                ),
                                                ident: `Point2d`,
                                                entity_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    933,
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
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `points`,
                                                        token_idx: TokenIdx(
                                                            926,
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
                                                                value: 160,
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
                                                    ident: `points`,
                                                    access_start: TokenIdx(
                                                        927,
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
                                                ty: 2,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 3,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        925,
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
                                                927,
                                            ),
                                        },
                                        ty: 2,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        931,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        932,
                                    ),
                                },
                            ),
                            return_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        934,
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::NewBoxList {
                                                        caller: None,
                                                        lbox_token_idx: TokenIdx(
                                                            928,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            929,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    },
                                                    Expr::Application {
                                                        function: 0,
                                                        argument: 1,
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
                                                            930,
                                                        ),
                                                        ident: `Point2d`,
                                                        entity_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            933,
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `points`,
                                                                token_idx: TokenIdx(
                                                                    926,
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
                                                                        value: 160,
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
                                                            ident: `points`,
                                                            access_start: TokenIdx(
                                                                927,
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
                                                        ty: 2,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: OutputType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `points`,
                                            token_idx: TokenIdx(
                                                938,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                939,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    940,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                941,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                942,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `points`,
                                            token_idx: TokenIdx(
                                                946,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `N`,
                                            token_idx: TokenIdx(
                                                948,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                950,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                949,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                2,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                947,
                                            ),
                                            items: ArenaIdxRange(
                                                5..6,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                951,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `points`,
                                            token_idx: TokenIdx(
                                                955,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `N`,
                                            token_idx: TokenIdx(
                                                957,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                959,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 8,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                958,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                7,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                956,
                                            ),
                                            items: ArenaIdxRange(
                                                10..11,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                960,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `p0`,
                                            token_idx: TokenIdx(
                                                964,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `p2`,
                                            token_idx: TokenIdx(
                                                968,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 13,
                                            dot_token_idx: TokenIdx(
                                                965,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    966,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 14,
                                            dot_token_idx: TokenIdx(
                                                969,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    970,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 15,
                                            opr: PureClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                967,
                                            ),
                                            ropd: 16,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                963,
                                            ),
                                            item: 17,
                                            rpar_token_idx: TokenIdx(
                                                971,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                973,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `p0`,
                                            token_idx: TokenIdx(
                                                976,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `p2`,
                                            token_idx: TokenIdx(
                                                980,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 20,
                                            dot_token_idx: TokenIdx(
                                                977,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    978,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                981,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    982,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 22,
                                            opr: PureClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                979,
                                            ),
                                            ropd: 23,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                975,
                                            ),
                                            item: 24,
                                            rpar_token_idx: TokenIdx(
                                                983,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                985,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 18,
                                            opr: PureClosed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                972,
                                            ),
                                            ropd: 19,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 25,
                                            opr: PureClosed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                984,
                                            ),
                                            ropd: 26,
                                        },
                                        Expr::FunctionCall {
                                            function: 12,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                962,
                                            ),
                                            arguments: ArenaIdxRange(
                                                27..29,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                987,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..4,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                961,
                                            ),
                                            ident: `Point2d`,
                                            entity_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    935,
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
                                                        937,
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
                                                    943,
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
                                                        945,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                6,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    952,
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
                                                        954,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                11,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 29,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `N`,
                                                    token_idx: TokenIdx(
                                                        936,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `p0`,
                                                    token_idx: TokenIdx(
                                                        944,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `p2`,
                                                    token_idx: TokenIdx(
                                                        953,
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
                                                            value: 213,
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
                                                            value: 246,
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
                                                            value: 247,
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
                                                ident: `points`,
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
                                                ident: `N`,
                                                access_start: TokenIdx(
                                                    937,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            988,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `p0`,
                                                access_start: TokenIdx(
                                                    945,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            988,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `p2`,
                                                access_start: TokenIdx(
                                                    954,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            988,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 2,
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
                                        expr: 30,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            30,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                            ast_idx: 214,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
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
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    993,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    997,
                                                ),
                                                items: ArenaIdxRange(
                                                    2..2,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    998,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
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
                                                    994,
                                                ),
                                                ident: `ConnectedComponent`,
                                                entity_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    999,
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
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            991,
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
                                                                value: 192,
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        992,
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
                                            kind: OutputType,
                                            expr: 4,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        990,
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
                                                992,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        995,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        996,
                                    ),
                                },
                            ),
                            return_ty: Ok(
                                OutputTypeExpr {
                                    expr: 4,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        1000,
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
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
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            993,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::NewBoxList {
                                                        caller: None,
                                                        lbox_token_idx: TokenIdx(
                                                            997,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            998,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
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
                                                            994,
                                                        ),
                                                        ident: `ConnectedComponent`,
                                                        entity_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            999,
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    991,
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
                                                                        value: 192,
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                992,
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
                                                    kind: OutputType,
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
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
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                1005,
                                            ),
                                            items: ArenaIdxRange(
                                                0..0,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1006,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1008,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1009,
                                            ),
                                        },
                                        Expr::Application {
                                            function: 1,
                                            argument: 2,
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    1014,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 191,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::FunctionCall {
                                            function: 4,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1015,
                                            ),
                                            arguments: ArenaIdxRange(
                                                5..5,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1016,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1018,
                                            ),
                                        ),
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                1020,
                                            ),
                                            ident: `i`,
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                7,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 6,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1019,
                                            ),
                                            ropd: 7,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1022,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 8,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1021,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                1027,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                1028,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `mask`,
                                                token_idx: TokenIdx(
                                                    1029,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1031,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                7,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1033,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 13,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1032,
                                            ),
                                            ropd: 14,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                12,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1030,
                                            ),
                                            items: ArenaIdxRange(
                                                15..16,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1034,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                1038,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 17,
                                            dot_token_idx: TokenIdx(
                                                1039,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `mask`,
                                                token_idx: TokenIdx(
                                                    1040,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1042,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                7,
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                18,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1041,
                                            ),
                                            items: ArenaIdxRange(
                                                19..20,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1043,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_ur`,
                                            token_idx: TokenIdx(
                                                1047,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1049,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 21,
                                            opr: PureClosed(
                                                Shl,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1048,
                                            ),
                                            ropd: 22,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_dr`,
                                            token_idx: TokenIdx(
                                                1053,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1055,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 24,
                                            opr: PureClosed(
                                                Shl,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1054,
                                            ),
                                            ropd: 25,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `boundary_unsearched`,
                                            token_idx: TokenIdx(
                                                1056,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1058,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                7,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_ur`,
                                            token_idx: TokenIdx(
                                                1062,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_dr`,
                                            token_idx: TokenIdx(
                                                1064,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 29,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1063,
                                            ),
                                            ropd: 30,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_ul`,
                                            token_idx: TokenIdx(
                                                1066,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 31,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1065,
                                            ),
                                            ropd: 32,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_dl`,
                                            token_idx: TokenIdx(
                                                1068,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 33,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1067,
                                            ),
                                            ropd: 34,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_ur`,
                                            token_idx: TokenIdx(
                                                1074,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_dr`,
                                            token_idx: TokenIdx(
                                                1076,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 36,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1075,
                                            ),
                                            ropd: 37,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_ul`,
                                            token_idx: TokenIdx(
                                                1078,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 38,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1077,
                                            ),
                                            ropd: 39,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `r_dl`,
                                            token_idx: TokenIdx(
                                                1080,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 40,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1079,
                                            ),
                                            ropd: 41,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                1073,
                                            ),
                                            item: 42,
                                            rpar_token_idx: TokenIdx(
                                                1081,
                                            ),
                                        },
                                        Expr::PrefixOpn {
                                            opr: BitNot,
                                            opr_token_idx: TokenIdx(
                                                1072,
                                            ),
                                            opd: 43,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                1061,
                                            ),
                                            item: 35,
                                            rpar_token_idx: TokenIdx(
                                                1069,
                                            ),
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                1071,
                                            ),
                                            item: 44,
                                            rpar_token_idx: TokenIdx(
                                                1082,
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                27,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1057,
                                            ),
                                            items: ArenaIdxRange(
                                                28..29,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1059,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 45,
                                            opr: PureClosed(
                                                BitOr,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1070,
                                            ),
                                            ropd: 46,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 47,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1060,
                                            ),
                                            ropd: 48,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1084,
                                            ),
                                        ),
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                1086,
                                            ),
                                            ident: `k`,
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                51,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 50,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1085,
                                            ),
                                            ropd: 51,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1088,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 52,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1087,
                                            ),
                                            ropd: 53,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `boundary_unsearched`,
                                            token_idx: TokenIdx(
                                                1091,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `k`,
                                            token_idx: TokenIdx(
                                                1093,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                51,
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                55,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1092,
                                            ),
                                            items: ArenaIdxRange(
                                                56..57,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1094,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                1100,
                                            ),
                                            items: ArenaIdxRange(
                                                58..58,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1101,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 58,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1103,
                                            ),
                                            arguments: ArenaIdxRange(
                                                59..59,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1104,
                                            ),
                                        },
                                        Expr::Application {
                                            function: 59,
                                            argument: 60,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `k`,
                                            token_idx: TokenIdx(
                                                1109,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                51,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `boundary_unsearched`,
                                            token_idx: TokenIdx(
                                                1114,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `k`,
                                            token_idx: TokenIdx(
                                                1116,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                51,
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                63,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1115,
                                            ),
                                            items: ArenaIdxRange(
                                                64..65,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1117,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            self_expr: 65,
                                            dot_token_idx: TokenIdx(
                                                1118,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ctz`,
                                                token_idx: TokenIdx(
                                                    1119,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1120,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                66..66,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1121,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                1126,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 67,
                                            dot_token_idx: TokenIdx(
                                                1127,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `mask`,
                                                token_idx: TokenIdx(
                                                    1128,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1130,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1132,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 69,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1131,
                                            ),
                                            ropd: 70,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                68,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1129,
                                            ),
                                            items: ArenaIdxRange(
                                                71..72,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1133,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                1138,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 73,
                                            dot_token_idx: TokenIdx(
                                                1139,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `mask`,
                                                token_idx: TokenIdx(
                                                    1140,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1142,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                74,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1141,
                                            ),
                                            items: ArenaIdxRange(
                                                75..76,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1143,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `row_above`,
                                            token_idx: TokenIdx(
                                                1150,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `row_below`,
                                            token_idx: TokenIdx(
                                                1152,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                1154,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 77,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1149,
                                            ),
                                            arguments: ArenaIdxRange(
                                                78..81,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1155,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1159,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                1163,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `inward_direction`,
                                            token_idx: TokenIdx(
                                                1167,
                                            ),
                                            current_symbol_idx: 13,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1172,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                1177,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                1182,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                1188,
                                            ),
                                        ),
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1187,
                                            ),
                                            opd: 88,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1194,
                                            ),
                                        ),
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1193,
                                            ),
                                            opd: 90,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1200,
                                            ),
                                        ),
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1199,
                                            ),
                                            opd: 92,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1205,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i0`,
                                            token_idx: TokenIdx(
                                                1207,
                                            ),
                                            current_symbol_idx: 14,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                1209,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `j0`,
                                            token_idx: TokenIdx(
                                                1211,
                                            ),
                                            current_symbol_idx: 15,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 13,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 94,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1206,
                                            ),
                                            ropd: 95,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 96,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1210,
                                            ),
                                            ropd: 97,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `inward_direction`,
                                            token_idx: TokenIdx(
                                                1213,
                                            ),
                                            current_symbol_idx: 13,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dir0`,
                                            token_idx: TokenIdx(
                                                1215,
                                            ),
                                            current_symbol_idx: 16,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 98,
                                            opr: ShortcuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1208,
                                            ),
                                            ropd: 99,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 100,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1214,
                                            ),
                                            ropd: 101,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 102,
                                            opr: ShortcuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1212,
                                            ),
                                            ropd: 103,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                1204,
                                            ),
                                            item: 104,
                                            rpar_token_idx: TokenIdx(
                                                1216,
                                            ),
                                        },
                                        Expr::PrefixOpn {
                                            opr: Not,
                                            opr_token_idx: TokenIdx(
                                                1203,
                                            ),
                                            opd: 105,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `row_above`,
                                            token_idx: TokenIdx(
                                                1223,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `row_below`,
                                            token_idx: TokenIdx(
                                                1225,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                1227,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `inward_direction`,
                                            token_idx: TokenIdx(
                                                1229,
                                            ),
                                            current_symbol_idx: 13,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 107,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1222,
                                            ),
                                            arguments: ArenaIdxRange(
                                                108..112,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1230,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `inward_direction`,
                                            token_idx: TokenIdx(
                                                1236,
                                            ),
                                            current_symbol_idx: 13,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `outward_direction`,
                                            token_idx: TokenIdx(
                                                1238,
                                            ),
                                            current_symbol_idx: 23,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 21,
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 113,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1235,
                                            ),
                                            arguments: ArenaIdxRange(
                                                114..116,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1239,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `boundary_unsearched`,
                                            token_idx: TokenIdx(
                                                1240,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1242,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `boundary_unsearched`,
                                            token_idx: TokenIdx(
                                                1245,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1247,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1253,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                1255,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 121,
                                            opr: PureClosed(
                                                Shl,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1254,
                                            ),
                                            ropd: 122,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                1252,
                                            ),
                                            item: 123,
                                            rpar_token_idx: TokenIdx(
                                                1256,
                                            ),
                                        },
                                        Expr::PrefixOpn {
                                            opr: BitNot,
                                            opr_token_idx: TokenIdx(
                                                1251,
                                            ),
                                            opd: 124,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                1250,
                                            ),
                                            item: 125,
                                            rpar_token_idx: TokenIdx(
                                                1257,
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                119,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1246,
                                            ),
                                            items: ArenaIdxRange(
                                                120..121,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1248,
                                            ),
                                        },
                                        Expr::PrefixOpn {
                                            opr: Ref,
                                            opr_token_idx: TokenIdx(
                                                1249,
                                            ),
                                            opd: 126,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                117,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                1241,
                                            ),
                                            items: ArenaIdxRange(
                                                118..119,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                1243,
                                            ),
                                        },
                                        Expr::Application {
                                            function: 127,
                                            argument: 128,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 129,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1244,
                                            ),
                                            ropd: 130,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `angle_change`,
                                            token_idx: TokenIdx(
                                                1259,
                                            ),
                                            current_symbol_idx: 24,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 22,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1265,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `prev_angle_change1`,
                                            token_idx: TokenIdx(
                                                1262,
                                            ),
                                            current_symbol_idx: 17,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 15,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1264,
                                            ),
                                            opd: 133,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1270,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `prev_angle_change2`,
                                            token_idx: TokenIdx(
                                                1267,
                                            ),
                                            current_symbol_idx: 18,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1269,
                                            ),
                                            opd: 136,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 134,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1263,
                                            ),
                                            ropd: 135,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 137,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1268,
                                            ),
                                            ropd: 138,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_streak`,
                                            token_idx: TokenIdx(
                                                1272,
                                            ),
                                            current_symbol_idx: 22,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 20,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1274,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 139,
                                            opr: ShortcuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1266,
                                            ),
                                            ropd: 140,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 141,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1273,
                                            ),
                                            ropd: 142,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1279,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `prev_streak1`,
                                            token_idx: TokenIdx(
                                                1276,
                                            ),
                                            current_symbol_idx: 20,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 18,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1278,
                                            ),
                                            opd: 145,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 143,
                                            opr: ShortcuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1271,
                                            ),
                                            ropd: 144,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 146,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1277,
                                            ),
                                            ropd: 147,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `prev_streak2`,
                                            token_idx: TokenIdx(
                                                1281,
                                            ),
                                            current_symbol_idx: 21,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 19,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1283,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 148,
                                            opr: ShortcuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1275,
                                            ),
                                            ropd: 149,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 150,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1282,
                                            ),
                                            ropd: 151,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 152,
                                            opr: ShortcuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1280,
                                            ),
                                            ropd: 153,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `contour`,
                                            token_idx: TokenIdx(
                                                1285,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 5,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `contour`,
                                            token_idx: TokenIdx(
                                                1293,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_expr: 155,
                                            dot_token_idx: TokenIdx(
                                                1286,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lastx`,
                                                token_idx: TokenIdx(
                                                    1287,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1288,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                156..156,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1289,
                                            ),
                                        },
                                        Expr::ApplicationOrFunctionCall {
                                            function: 156,
                                            lpar_token_idx: TokenIdx(
                                                1292,
                                            ),
                                            argument: 157,
                                            rpar_token_idx: TokenIdx(
                                                1294,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 158,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1290,
                                            ),
                                            ropd: 159,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `contour`,
                                            token_idx: TokenIdx(
                                                1295,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 7,
                                            entity_path: None,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                1303,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `j`,
                                            token_idx: TokenIdx(
                                                1305,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 162,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1302,
                                            ),
                                            arguments: ArenaIdxRange(
                                                163..165,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1306,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            self_expr: 161,
                                            dot_token_idx: TokenIdx(
                                                1296,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `push`,
                                                token_idx: TokenIdx(
                                                    1297,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1298,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                165..166,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1307,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1311,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `prev_streak2`,
                                            token_idx: TokenIdx(
                                                1308,
                                            ),
                                            current_symbol_idx: 21,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 19,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1310,
                                            ),
                                            opd: 167,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 168,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1309,
                                            ),
                                            ropd: 169,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1315,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `prev_streak1`,
                                            token_idx: TokenIdx(
                                                1312,
                                            ),
                                            current_symbol_idx: 20,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 18,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1314,
                                            ),
                                            opd: 171,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 172,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1313,
                                            ),
                                            ropd: 173,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_streak`,
                                            token_idx: TokenIdx(
                                                1411,
                                            ),
                                            current_symbol_idx: 22,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 20,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1413,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 175,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1412,
                                            ),
                                            ropd: 176,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `prev_angle_change2`,
                                            token_idx: TokenIdx(
                                                1414,
                                            ),
                                            current_symbol_idx: 18,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `prev_angle_change1`,
                                            token_idx: TokenIdx(
                                                1416,
                                            ),
                                            current_symbol_idx: 17,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 15,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 178,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1415,
                                            ),
                                            ropd: 179,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `prev_angle_change1`,
                                            token_idx: TokenIdx(
                                                1417,
                                            ),
                                            current_symbol_idx: 17,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 15,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `angle_change`,
                                            token_idx: TokenIdx(
                                                1419,
                                            ),
                                            current_symbol_idx: 24,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 22,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 181,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1418,
                                            ),
                                            ropd: 182,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `inward_direction`,
                                            token_idx: TokenIdx(
                                                1494,
                                            ),
                                            current_symbol_idx: 13,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `outward_direction`,
                                            token_idx: TokenIdx(
                                                1496,
                                            ),
                                            current_symbol_idx: 23,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 21,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 184,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1495,
                                            ),
                                            ropd: 185,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1501,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `current_streak`,
                                            token_idx: TokenIdx(
                                                1498,
                                            ),
                                            current_symbol_idx: 22,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 20,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1500,
                                            ),
                                            opd: 187,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 188,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1499,
                                            ),
                                            ropd: 189,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_streak`,
                                            token_idx: TokenIdx(
                                                1503,
                                            ),
                                            current_symbol_idx: 22,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 20,
                                            },
                                        },
                                        Expr::SuffixOpn {
                                            opd: 191,
                                            punctuation: Incr,
                                            punctuation_token_idx: TokenIdx(
                                                1504,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1509,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `prev_angle_change1`,
                                            token_idx: TokenIdx(
                                                1506,
                                            ),
                                            current_symbol_idx: 17,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 15,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                1508,
                                            ),
                                            opd: 193,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_streak`,
                                            token_idx: TokenIdx(
                                                1511,
                                            ),
                                            current_symbol_idx: 22,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 20,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1513,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 194,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1507,
                                            ),
                                            ropd: 195,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 196,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1512,
                                            ),
                                            ropd: 197,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `prev_streak1`,
                                            token_idx: TokenIdx(
                                                1515,
                                            ),
                                            current_symbol_idx: 20,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 18,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                1517,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 198,
                                            opr: ShortcuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1510,
                                            ),
                                            ropd: 199,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 200,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1516,
                                            ),
                                            ropd: 201,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 202,
                                            opr: ShortcuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                1514,
                                            ),
                                            ropd: 203,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `contour`,
                                            token_idx: TokenIdx(
                                                1519,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_expr: 205,
                                            dot_token_idx: TokenIdx(
                                                1520,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `popx`,
                                                token_idx: TokenIdx(
                                                    1521,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1522,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                206..206,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1523,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `result`,
                                            token_idx: TokenIdx(
                                                1525,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 8,
                                            entity_path: Some(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                1531,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `contour`,
                                            token_idx: TokenIdx(
                                                1533,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 208,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1530,
                                            ),
                                            arguments: ArenaIdxRange(
                                                209..211,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1534,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            self_expr: 207,
                                            dot_token_idx: TokenIdx(
                                                1526,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `push`,
                                                token_idx: TokenIdx(
                                                    1527,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                1528,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                211..212,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                1535,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `result`,
                                            token_idx: TokenIdx(
                                                1537,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                41..46,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                1007,
                                            ),
                                            ident: `RawContour`,
                                            entity_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                1102,
                                            ),
                                            ident: `Point2d`,
                                            entity_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                1148,
                                            ),
                                            ident: `get_inward_direction`,
                                            entity_path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                1221,
                                            ),
                                            ident: `get_outward_direction`,
                                            entity_path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                1234,
                                            ),
                                            ident: `get_angle_change`,
                                            entity_path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                1291,
                                            ),
                                            ident: `get_concave_middle_point`,
                                            entity_path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                1299,
                                            ),
                                            ident: `Point2d`,
                                            entity_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        },
                                        EntityPathExpr::Subentity {
                                            parent: 6,
                                            scope_resolution_token: ScopeResolutionToken {
                                                token_idx: TokenIdx(
                                                    1300,
                                                ),
                                            },
                                            ident_token: Ok(
                                                IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 266,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        1301,
                                                    ),
                                                },
                                            ),
                                            path: Err(
                                                Original(
                                                    EntityTree {
                                                        token_idx: TokenIdx(
                                                            1301,
                                                        ),
                                                        error: NoSubentity,
                                                    },
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                1529,
                                            ),
                                            ident: `RawContour`,
                                            entity_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1024,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        1026,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                16,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1035,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        1037,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                20,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1044,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        1046,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                23,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1050,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        1052,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                26,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 49,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 160,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 166,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 170,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 174,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        1261,
                                                    ),
                                                },
                                                condition: Ok(
                                                    154,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            1284,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        5..9,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 177,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 180,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 183,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 192,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1218,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 21,
                                                    variables: ArenaIdxRange(
                                                        23..24,
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
                                                        1220,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                112,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1231,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 22,
                                                    variables: ArenaIdxRange(
                                                        24..25,
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
                                                        1233,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                116,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 131,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        1258,
                                                    ),
                                                },
                                                condition: Ok(
                                                    132,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            1260,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        9..13,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Match,
                                        Stmt::Eval {
                                            expr_idx: 186,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        1497,
                                                    ),
                                                },
                                                condition: Ok(
                                                    190,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            1502,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 206,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1096,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 6,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        1099,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                61,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1105,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 7,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        1108,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                62,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1110,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 8,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        1113,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                66,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1122,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 9,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        1125,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                72,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1134,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 10,
                                                    variables: ArenaIdxRange(
                                                        12..13,
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
                                                        1137,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                76,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1144,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 11,
                                                    variables: ArenaIdxRange(
                                                        13..14,
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
                                                        1147,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                81,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1156,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 12,
                                                    variables: ArenaIdxRange(
                                                        14..15,
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
                                                        1158,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                82,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1160,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 13,
                                                    variables: ArenaIdxRange(
                                                        15..16,
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
                                                        1162,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                83,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1164,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 14,
                                                    variables: ArenaIdxRange(
                                                        16..17,
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
                                                        1166,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                84,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1168,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 15,
                                                    variables: ArenaIdxRange(
                                                        17..18,
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
                                                        1171,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                85,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1173,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 16,
                                                    variables: ArenaIdxRange(
                                                        18..19,
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
                                                        1176,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                86,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1178,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 17,
                                                    variables: ArenaIdxRange(
                                                        19..20,
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
                                                        1181,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                87,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1183,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 18,
                                                    variables: ArenaIdxRange(
                                                        20..21,
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
                                                        1186,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                89,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1189,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 19,
                                                    variables: ArenaIdxRange(
                                                        21..22,
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
                                                        1192,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                91,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1195,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 20,
                                                    variables: ArenaIdxRange(
                                                        22..23,
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
                                                        1198,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                93,
                                            ),
                                        },
                                        Stmt::DoWhile {
                                            do_token: DoToken {
                                                token_idx: TokenIdx(
                                                    1201,
                                                ),
                                            },
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    1202,
                                                ),
                                            },
                                            condition: Ok(
                                                106,
                                            ),
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        1217,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    14..21,
                                                ),
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        1505,
                                                    ),
                                                },
                                                condition: Ok(
                                                    204,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            1518,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        21..22,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 212,
                                        },
                                        Stmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    1090,
                                                ),
                                            },
                                            condition: Ok(
                                                57,
                                            ),
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        1095,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    22..40,
                                                ),
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1001,
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
                                                        1004,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                3,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    1010,
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
                                                        1013,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                5,
                                            ),
                                        },
                                        Stmt::ForBetween {
                                            for_token: ForToken {
                                                token_idx: TokenIdx(
                                                    1017,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    1020,
                                                ),
                                                frame_var_expr_idx: 7,
                                                frame_var_ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 157,
                                                        },
                                                    ),
                                                ),
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            6,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            9,
                                                        ),
                                                        kind: UpperClosed,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        1023,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..5,
                                                ),
                                            ),
                                        },
                                        Stmt::ForBetween {
                                            for_token: ForToken {
                                                token_idx: TokenIdx(
                                                    1083,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    1086,
                                                ),
                                                frame_var_expr_idx: 51,
                                                frame_var_ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 173,
                                                        },
                                                    ),
                                                ),
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            50,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            53,
                                                        ),
                                                        kind: UpperClosed,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        1089,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    40..41,
                                                ),
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    1536,
                                                ),
                                            },
                                            result: Ok(
                                                213,
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `result`,
                                                    token_idx: TokenIdx(
                                                        1003,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `boundary_unsearched`,
                                                    token_idx: TokenIdx(
                                                        1012,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `r_ur`,
                                                    token_idx: TokenIdx(
                                                        1025,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `r_dr`,
                                                    token_idx: TokenIdx(
                                                        1036,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `r_ul`,
                                                    token_idx: TokenIdx(
                                                        1045,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `r_dl`,
                                                    token_idx: TokenIdx(
                                                        1051,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `contour`,
                                                    token_idx: TokenIdx(
                                                        1098,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        1107,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        1112,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `row_above`,
                                                    token_idx: TokenIdx(
                                                        1124,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `row_below`,
                                                    token_idx: TokenIdx(
                                                        1136,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `inward_direction`,
                                                    token_idx: TokenIdx(
                                                        1146,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `i0`,
                                                    token_idx: TokenIdx(
                                                        1157,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `j0`,
                                                    token_idx: TokenIdx(
                                                        1161,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dir0`,
                                                    token_idx: TokenIdx(
                                                        1165,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `prev_angle_change1`,
                                                    token_idx: TokenIdx(
                                                        1170,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `prev_angle_change2`,
                                                    token_idx: TokenIdx(
                                                        1175,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `total_angle_change`,
                                                    token_idx: TokenIdx(
                                                        1180,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `prev_streak1`,
                                                    token_idx: TokenIdx(
                                                        1185,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `prev_streak2`,
                                                    token_idx: TokenIdx(
                                                        1191,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `current_streak`,
                                                    token_idx: TokenIdx(
                                                        1197,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `outward_direction`,
                                                    token_idx: TokenIdx(
                                                        1219,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `angle_change`,
                                                    token_idx: TokenIdx(
                                                        1232,
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
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 184,
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
                                                            value: 248,
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
                                                            value: 249,
                                                        },
                                                    ),
                                                ),
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 250,
                                                        },
                                                    ),
                                                ),
                                                3,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 251,
                                                        },
                                                    ),
                                                ),
                                                4,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 252,
                                                        },
                                                    ),
                                                ),
                                                5,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 253,
                                                        },
                                                    ),
                                                ),
                                                6,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 157,
                                                        },
                                                    ),
                                                ),
                                                7,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 174,
                                                        },
                                                    ),
                                                ),
                                                8,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 229,
                                                        },
                                                    ),
                                                ),
                                                9,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 230,
                                                        },
                                                    ),
                                                ),
                                                10,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 241,
                                                        },
                                                    ),
                                                ),
                                                11,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 254,
                                                        },
                                                    ),
                                                ),
                                                12,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 255,
                                                        },
                                                    ),
                                                ),
                                                13,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 256,
                                                        },
                                                    ),
                                                ),
                                                14,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 257,
                                                        },
                                                    ),
                                                ),
                                                15,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 258,
                                                        },
                                                    ),
                                                ),
                                                16,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 259,
                                                        },
                                                    ),
                                                ),
                                                17,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 260,
                                                        },
                                                    ),
                                                ),
                                                18,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 261,
                                                        },
                                                    ),
                                                ),
                                                19,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 262,
                                                        },
                                                    ),
                                                ),
                                                20,
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
                                                21,
                                            ),
                                        ],
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 264,
                                                        },
                                                    ),
                                                ),
                                                22,
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
                                            PatternSymbol::Atom(
                                                11,
                                            ),
                                            PatternSymbol::Atom(
                                                12,
                                            ),
                                            PatternSymbol::Atom(
                                                13,
                                            ),
                                            PatternSymbol::Atom(
                                                14,
                                            ),
                                            PatternSymbol::Atom(
                                                15,
                                            ),
                                            PatternSymbol::Atom(
                                                16,
                                            ),
                                            PatternSymbol::Atom(
                                                17,
                                            ),
                                            PatternSymbol::Atom(
                                                18,
                                            ),
                                            PatternSymbol::Atom(
                                                19,
                                            ),
                                            PatternSymbol::Atom(
                                                20,
                                            ),
                                            PatternSymbol::Atom(
                                                21,
                                            ),
                                            PatternSymbol::Atom(
                                                22,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                ident: `cc`,
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
                                                ident: `result`,
                                                access_start: TokenIdx(
                                                    1004,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1538,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `boundary_unsearched`,
                                                access_start: TokenIdx(
                                                    1013,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1538,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `i`,
                                                access_start: TokenIdx(
                                                    1024,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1083,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable(
                                                    7,
                                                ),
                                            },
                                            CurrentSymbol {
                                                ident: `r_ur`,
                                                access_start: TokenIdx(
                                                    1026,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1083,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `r_dr`,
                                                access_start: TokenIdx(
                                                    1037,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1083,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `r_ul`,
                                                access_start: TokenIdx(
                                                    1046,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1083,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `r_dl`,
                                                access_start: TokenIdx(
                                                    1052,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1083,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `k`,
                                                access_start: TokenIdx(
                                                    1090,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable(
                                                    51,
                                                ),
                                            },
                                            CurrentSymbol {
                                                ident: `contour`,
                                                access_start: TokenIdx(
                                                    1099,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `i`,
                                                access_start: TokenIdx(
                                                    1108,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `j`,
                                                access_start: TokenIdx(
                                                    1113,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `row_above`,
                                                access_start: TokenIdx(
                                                    1125,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `row_below`,
                                                access_start: TokenIdx(
                                                    1137,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 10,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `inward_direction`,
                                                access_start: TokenIdx(
                                                    1147,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 11,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `i0`,
                                                access_start: TokenIdx(
                                                    1158,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 12,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `j0`,
                                                access_start: TokenIdx(
                                                    1162,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 13,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `dir0`,
                                                access_start: TokenIdx(
                                                    1166,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 14,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `prev_angle_change1`,
                                                access_start: TokenIdx(
                                                    1171,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 15,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `prev_angle_change2`,
                                                access_start: TokenIdx(
                                                    1176,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 16,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `total_angle_change`,
                                                access_start: TokenIdx(
                                                    1181,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 17,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `prev_streak1`,
                                                access_start: TokenIdx(
                                                    1186,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 18,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `prev_streak2`,
                                                access_start: TokenIdx(
                                                    1192,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 19,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `current_streak`,
                                                access_start: TokenIdx(
                                                    1198,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1536,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 20,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `outward_direction`,
                                                access_start: TokenIdx(
                                                    1220,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1505,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 21,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `angle_change`,
                                                access_start: TokenIdx(
                                                    1233,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            1505,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 22,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        FrameVariable,
                                        FrameVariable,
                                    ],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 214,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            214,
                        ),
                    },
                ),
            ),
            ImplBlock(
                ImplBlockDecl::TypeImplBlock(
                    TypeImplBlockDecl {
                        ast_idx: 200,
                        impl_block: ImplBlock {
                            id: ImplBlockId {
                                module_path: `mnist_classifier::raw_contour`,
                                impl_block_kind: ImplBlockKind::Type {
                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                },
                            },
                            ast_idx: 200,
                            body: ArenaIdxRange(
                                27..34,
                            ),
                            variant: ImplBlockVariant::Type {
                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            },
                        },
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                42,
                            ),
                        },
                        implicit_parameter_decl_list: None,
                        ty: TypeExpr {
                            expr: 0,
                        },
                        eol_colon: Ok(
                            EolColonToken {
                                token_idx: TokenIdx(
                                    44,
                                ),
                            },
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    DeclExprPath::ImplBlock(
                                        ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
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
                                                43,
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
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath {
                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ident: `line_segment_sketch`,
                                    ty_item_kind: Memo,
                                },
                            ),
                            decl: TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ident: `line_segment_sketch`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ident: `line_segment_sketch`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `line_segment_sketch`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ast_idx: 200,
                                        body: ArenaIdxRange(
                                            27..34,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        },
                                    },
                                    ast_idx: 27,
                                    ident: `line_segment_sketch`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 27,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
                                                            },
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
                                                                    43,
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
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        },
                                                    },
                                                    ident: `line_segment_sketch`,
                                                },
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
                                                        48,
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
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            47,
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
                                            49,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        impl_block_kind: ImplBlockKind::Type {
                                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        },
                                                                    },
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
                                                                            43,
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
                                                            },
                                                            ident: `line_segment_sketch`,
                                                        },
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
                                                                48,
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
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    },
                                                },
                                                ident: `line_segment_sketch`,
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: None,
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    54,
                                                ),
                                            ),
                                            Expr::Literal(
                                                TokenIdx(
                                                    56,
                                                ),
                                            ),
                                            Expr::FunctionCall {
                                                function: 0,
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    53,
                                                ),
                                                arguments: ArenaIdxRange(
                                                    1..3,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    57,
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
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    50,
                                                ),
                                                ident: `LineSegmentSketch`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            },
                                            EntityPathExpr::Subentity {
                                                parent: 0,
                                                scope_resolution_token: ScopeResolutionToken {
                                                    token_idx: TokenIdx(
                                                        51,
                                                    ),
                                                },
                                                ident_token: Ok(
                                                    IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 195,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                    },
                                                ),
                                                path: Err(
                                                    Original(
                                                        EntityTree {
                                                            token_idx: TokenIdx(
                                                                52,
                                                            ),
                                                            error: NoSubentity,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [
                                            Stmt::Eval {
                                                expr_idx: 3,
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
                                            expr: 4,
                                        },
                                    ],
                                },
                            },
                            body: Ok(
                                4,
                            ),
                        },
                    ),
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath {
                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ident: `bounding_box`,
                                    ty_item_kind: Memo,
                                },
                            ),
                            decl: TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ident: `bounding_box`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ident: `bounding_box`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `bounding_box`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ast_idx: 200,
                                        body: ArenaIdxRange(
                                            27..34,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        },
                                    },
                                    ast_idx: 28,
                                    ident: `bounding_box`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 28,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
                                                            },
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
                                                                    43,
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
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        },
                                                    },
                                                    ident: `bounding_box`,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                    ident: `BoundingBox`,
                                                    entity_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            60,
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
                                            62,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        impl_block_kind: ImplBlockKind::Type {
                                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        },
                                                                    },
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
                                                                            43,
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
                                                            },
                                                            ident: `bounding_box`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            entity_path: Some(
                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                            ident: `BoundingBox`,
                                                            entity_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    },
                                                },
                                                ident: `bounding_box`,
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    66,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 0,
                                                dot_token_idx: TokenIdx(
                                                    67,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        68,
                                                    ),
                                                },
                                            },
                                            Expr::Literal(
                                                TokenIdx(
                                                    70,
                                                ),
                                            ),
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    1,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    69,
                                                ),
                                                items: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    71,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `start_point`,
                                                token_idx: TokenIdx(
                                                    76,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 4,
                                                dot_token_idx: TokenIdx(
                                                    77,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `start_point`,
                                                token_idx: TokenIdx(
                                                    83,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 6,
                                                dot_token_idx: TokenIdx(
                                                    84,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `start_point`,
                                                token_idx: TokenIdx(
                                                    90,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 8,
                                                dot_token_idx: TokenIdx(
                                                    91,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        92,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `start_point`,
                                                token_idx: TokenIdx(
                                                    97,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 10,
                                                dot_token_idx: TokenIdx(
                                                    98,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                },
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    103,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 12,
                                                dot_token_idx: TokenIdx(
                                                    104,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        105,
                                                    ),
                                                },
                                            },
                                            Expr::FrameVarDecl {
                                                token_idx: TokenIdx(
                                                    101,
                                                ),
                                                ident: `i`,
                                                current_symbol_idx: 5,
                                                current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                    14,
                                                ),
                                            },
                                            Expr::MethodCall {
                                                self_expr: 13,
                                                dot_token_idx: TokenIdx(
                                                    106,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `ilen`,
                                                    token_idx: TokenIdx(
                                                        107,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    108,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    14..14,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    109,
                                                ),
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 14,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    102,
                                                ),
                                                ropd: 15,
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    114,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 17,
                                                dot_token_idx: TokenIdx(
                                                    115,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        116,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `i`,
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                                current_symbol_idx: 5,
                                                current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                    14,
                                                ),
                                            },
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    18,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    117,
                                                ),
                                                items: ArenaIdxRange(
                                                    19..20,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    119,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `xmin`,
                                                token_idx: TokenIdx(
                                                    122,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `point`,
                                                token_idx: TokenIdx(
                                                    126,
                                                ),
                                                current_symbol_idx: 6,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 22,
                                                dot_token_idx: TokenIdx(
                                                    127,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `xmin`,
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 21,
                                                dot_token_idx: TokenIdx(
                                                    123,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `min`,
                                                    token_idx: TokenIdx(
                                                        124,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    125,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    23..24,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    129,
                                                ),
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 24,
                                                opr: Assign(
                                                    None,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    121,
                                                ),
                                                ropd: 25,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `xmax`,
                                                token_idx: TokenIdx(
                                                    132,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `point`,
                                                token_idx: TokenIdx(
                                                    136,
                                                ),
                                                current_symbol_idx: 6,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 28,
                                                dot_token_idx: TokenIdx(
                                                    137,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        138,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `xmax`,
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 27,
                                                dot_token_idx: TokenIdx(
                                                    133,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `max`,
                                                    token_idx: TokenIdx(
                                                        134,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    135,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    29..30,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    139,
                                                ),
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 30,
                                                opr: Assign(
                                                    None,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    131,
                                                ),
                                                ropd: 31,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                                current_symbol_idx: 3,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `point`,
                                                token_idx: TokenIdx(
                                                    146,
                                                ),
                                                current_symbol_idx: 6,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 34,
                                                dot_token_idx: TokenIdx(
                                                    147,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    140,
                                                ),
                                                current_symbol_idx: 3,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 33,
                                                dot_token_idx: TokenIdx(
                                                    143,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `min`,
                                                    token_idx: TokenIdx(
                                                        144,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    145,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    35..36,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    149,
                                                ),
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 36,
                                                opr: Assign(
                                                    None,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    141,
                                                ),
                                                ropd: 37,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                                current_symbol_idx: 4,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `point`,
                                                token_idx: TokenIdx(
                                                    156,
                                                ),
                                                current_symbol_idx: 6,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 40,
                                                dot_token_idx: TokenIdx(
                                                    157,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        158,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    150,
                                                ),
                                                current_symbol_idx: 4,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 39,
                                                dot_token_idx: TokenIdx(
                                                    153,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `max`,
                                                    token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    155,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    41..42,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    159,
                                                ),
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 42,
                                                opr: Assign(
                                                    None,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    151,
                                                ),
                                                ropd: 43,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `xmin`,
                                                token_idx: TokenIdx(
                                                    165,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `xmax`,
                                                token_idx: TokenIdx(
                                                    167,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    172,
                                                ),
                                                current_symbol_idx: 3,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    174,
                                                ),
                                                current_symbol_idx: 4,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            Expr::FunctionCall {
                                                function: 46,
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    164,
                                                ),
                                                arguments: ArenaIdxRange(
                                                    47..49,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    168,
                                                ),
                                            },
                                            Expr::FunctionCall {
                                                function: 49,
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    171,
                                                ),
                                                arguments: ArenaIdxRange(
                                                    50..52,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    175,
                                                ),
                                            },
                                            Expr::FunctionCall {
                                                function: 45,
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    162,
                                                ),
                                                arguments: ArenaIdxRange(
                                                    52..54,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    177,
                                                ),
                                            },
                                            Expr::Block {
                                                stmts: ArenaIdxRange(
                                                    5..12,
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
                                                ident: `BoundingBox`,
                                                entity_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    163,
                                                ),
                                                ident: `ClosedRange`,
                                                entity_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                                ident: `ClosedRange`,
                                                entity_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        111,
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
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    20,
                                                ),
                                            },
                                            Stmt::Eval {
                                                expr_idx: 26,
                                            },
                                            Stmt::Eval {
                                                expr_idx: 32,
                                            },
                                            Stmt::Eval {
                                                expr_idx: 38,
                                            },
                                            Stmt::Eval {
                                                expr_idx: 44,
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        63,
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
                                                            65,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    3,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        72,
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
                                                            75,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    5,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        79,
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
                                                            82,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    7,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        86,
                                                    ),
                                                },
                                                let_variable_pattern: Ok(
                                                    LetVariablesPattern {
                                                        pattern: 3,
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
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            89,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    9,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        93,
                                                    ),
                                                },
                                                let_variable_pattern: Ok(
                                                    LetVariablesPattern {
                                                        pattern: 4,
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
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            96,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    11,
                                                ),
                                            },
                                            Stmt::ForBetween {
                                                for_token: ForToken {
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                },
                                                particulars: ForBetweenParticulars {
                                                    frame_var_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    frame_var_expr_idx: 14,
                                                    frame_var_ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 157,
                                                            },
                                                        ),
                                                    ),
                                                    range: ForBetweenRange {
                                                        initial_boundary: LoopBoundary {
                                                            bound_expr: None,
                                                            kind: LowerClosed,
                                                        },
                                                        final_boundary: LoopBoundary {
                                                            bound_expr: Some(
                                                                15,
                                                            ),
                                                            kind: UpperOpen,
                                                        },
                                                        step: Constant(
                                                            1,
                                                        ),
                                                    },
                                                },
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            110,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..5,
                                                    ),
                                                ),
                                            },
                                            Stmt::Return {
                                                return_token: ReturnToken {
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                },
                                                result: Ok(
                                                    54,
                                                ),
                                            },
                                        ],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            64,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `xmin`,
                                                        token_idx: TokenIdx(
                                                            74,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `xmax`,
                                                        token_idx: TokenIdx(
                                                            81,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            88,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            95,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            112,
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
                                                                value: 198,
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
                                                                value: 199,
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
                                                                value: 200,
                                                            },
                                                        ),
                                                    ),
                                                    2,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 201,
                                                            },
                                                        ),
                                                    ),
                                                    3,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 202,
                                                            },
                                                        ),
                                                    ),
                                                    4,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 203,
                                                            },
                                                        ),
                                                    ),
                                                    5,
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
                                                    ident: `start_point`,
                                                    access_start: TokenIdx(
                                                        65,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                178,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `xmin`,
                                                    access_start: TokenIdx(
                                                        75,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                178,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `xmax`,
                                                    access_start: TokenIdx(
                                                        82,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                178,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `ymin`,
                                                    access_start: TokenIdx(
                                                        89,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                178,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `ymax`,
                                                    access_start: TokenIdx(
                                                        96,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                178,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `i`,
                                                    access_start: TokenIdx(
                                                        111,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                160,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::FrameVariable(
                                                        14,
                                                    ),
                                                },
                                                CurrentSymbol {
                                                    ident: `point`,
                                                    access_start: TokenIdx(
                                                        113,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                160,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 5,
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
                                            expr: 55,
                                        },
                                    ],
                                },
                            },
                            body: Ok(
                                55,
                            ),
                        },
                    ),
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath {
                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ident: `relative_bounding_box`,
                                    ty_item_kind: Memo,
                                },
                            ),
                            decl: TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ident: `relative_bounding_box`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ident: `relative_bounding_box`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `relative_bounding_box`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ast_idx: 200,
                                        body: ArenaIdxRange(
                                            27..34,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        },
                                    },
                                    ast_idx: 29,
                                    ident: `relative_bounding_box`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 29,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
                                                            },
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
                                                                    43,
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
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        },
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
                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                    ident: `RelativeBoundingBox`,
                                                    entity_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            180,
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
                                            182,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        impl_block_kind: ImplBlockKind::Type {
                                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        },
                                                                    },
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
                                                                            43,
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
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
                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                entity_path_expr_arena: Arena {
                                                    data: [
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                181,
                                                            ),
                                                            ident: `RelativeBoundingBox`,
                                                            entity_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    },
                                                },
                                                ident: `relative_bounding_box`,
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    183,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 0,
                                                dot_token_idx: TokenIdx(
                                                    184,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                },
                                            },
                                            Expr::Field {
                                                owner: 1,
                                                dot_token_idx: TokenIdx(
                                                    186,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `raw_contours`,
                                                    token_idx: TokenIdx(
                                                        187,
                                                    ),
                                                },
                                            },
                                            Expr::Literal(
                                                TokenIdx(
                                                    189,
                                                ),
                                            ),
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    2,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    188,
                                                ),
                                                items: ArenaIdxRange(
                                                    3..4,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    190,
                                                ),
                                            },
                                            Expr::Field {
                                                owner: 4,
                                                dot_token_idx: TokenIdx(
                                                    191,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `bounding_box`,
                                                    token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                },
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    196,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 6,
                                                dot_token_idx: TokenIdx(
                                                    197,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `bounding_box`,
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 5,
                                                dot_token_idx: TokenIdx(
                                                    193,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `relative_bounding_box`,
                                                    token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    195,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    7..8,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    199,
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
                                                expr_idx: 8,
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
                                            expr: 9,
                                        },
                                    ],
                                },
                            },
                            body: Ok(
                                9,
                            ),
                        },
                    ),
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath {
                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ident: `contour_len`,
                                    ty_item_kind: Memo,
                                },
                            ),
                            decl: TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ident: `contour_len`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ident: `contour_len`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `contour_len`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ast_idx: 200,
                                        body: ArenaIdxRange(
                                            27..34,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        },
                                    },
                                    ast_idx: 30,
                                    ident: `contour_len`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 30,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
                                                            },
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
                                                                    43,
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
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        },
                                                    },
                                                    ident: `contour_len`,
                                                },
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
                                                        203,
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
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            202,
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
                                            204,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        impl_block_kind: ImplBlockKind::Type {
                                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        },
                                                                    },
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
                                                                            43,
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
                                                            },
                                                            ident: `contour_len`,
                                                        },
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
                                                                203,
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
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    },
                                                },
                                                ident: `contour_len`,
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Literal(
                                                TokenIdx(
                                                    209,
                                                ),
                                            ),
                                            Expr::Literal(
                                                TokenIdx(
                                                    211,
                                                ),
                                            ),
                                            Expr::FrameVarDecl {
                                                token_idx: TokenIdx(
                                                    213,
                                                ),
                                                ident: `i`,
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                    2,
                                                ),
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    215,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 3,
                                                dot_token_idx: TokenIdx(
                                                    216,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 1,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    212,
                                                ),
                                                ropd: 2,
                                            },
                                            Expr::MethodCall {
                                                self_expr: 4,
                                                dot_token_idx: TokenIdx(
                                                    218,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `ilen`,
                                                    token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    220,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    5..5,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    221,
                                                ),
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 5,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    214,
                                                ),
                                                ropd: 6,
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    226,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 8,
                                                dot_token_idx: TokenIdx(
                                                    227,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `i`,
                                                token_idx: TokenIdx(
                                                    230,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                    2,
                                                ),
                                            },
                                            Expr::Literal(
                                                TokenIdx(
                                                    232,
                                                ),
                                            ),
                                            Expr::BinaryOpn {
                                                lopd: 10,
                                                opr: PureClosed(
                                                    Sub,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    231,
                                                ),
                                                ropd: 11,
                                            },
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    9,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    229,
                                                ),
                                                items: ArenaIdxRange(
                                                    12..13,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    233,
                                                ),
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    237,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 14,
                                                dot_token_idx: TokenIdx(
                                                    238,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        239,
                                                    ),
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `i`,
                                                token_idx: TokenIdx(
                                                    241,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                    2,
                                                ),
                                            },
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    15,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    240,
                                                ),
                                                items: ArenaIdxRange(
                                                    16..17,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    242,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `a`,
                                                token_idx: TokenIdx(
                                                    246,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `b`,
                                                token_idx: TokenIdx(
                                                    250,
                                                ),
                                                current_symbol_idx: 3,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 18,
                                                dot_token_idx: TokenIdx(
                                                    247,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        248,
                                                    ),
                                                },
                                            },
                                            Expr::Field {
                                                owner: 19,
                                                dot_token_idx: TokenIdx(
                                                    251,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        252,
                                                    ),
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 20,
                                                opr: PureClosed(
                                                    Sub,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    249,
                                                ),
                                                ropd: 21,
                                            },
                                            Expr::Bracketed {
                                                lpar_token_idx: TokenIdx(
                                                    245,
                                                ),
                                                item: 22,
                                                rpar_token_idx: TokenIdx(
                                                    253,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `a`,
                                                token_idx: TokenIdx(
                                                    260,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `b`,
                                                token_idx: TokenIdx(
                                                    264,
                                                ),
                                                current_symbol_idx: 3,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 24,
                                                dot_token_idx: TokenIdx(
                                                    261,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        262,
                                                    ),
                                                },
                                            },
                                            Expr::Field {
                                                owner: 25,
                                                dot_token_idx: TokenIdx(
                                                    265,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        266,
                                                    ),
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 26,
                                                opr: PureClosed(
                                                    Sub,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    263,
                                                ),
                                                ropd: 27,
                                            },
                                            Expr::Bracketed {
                                                lpar_token_idx: TokenIdx(
                                                    259,
                                                ),
                                                item: 28,
                                                rpar_token_idx: TokenIdx(
                                                    267,
                                                ),
                                            },
                                            Expr::MethodCall {
                                                self_expr: 23,
                                                dot_token_idx: TokenIdx(
                                                    254,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `abs`,
                                                    token_idx: TokenIdx(
                                                        255,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    256,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    24..24,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    257,
                                                ),
                                            },
                                            Expr::MethodCall {
                                                self_expr: 29,
                                                dot_token_idx: TokenIdx(
                                                    268,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `abs`,
                                                    token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    270,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    30..30,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    271,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `contour_len`,
                                                token_idx: TokenIdx(
                                                    243,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 30,
                                                opr: PureClosed(
                                                    Add,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    258,
                                                ),
                                                ropd: 31,
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 32,
                                                opr: Assign(
                                                    Some(
                                                        Add,
                                                    ),
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    244,
                                                ),
                                                ropd: 33,
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    275,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 35,
                                                dot_token_idx: TokenIdx(
                                                    276,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        277,
                                                    ),
                                                },
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    279,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 37,
                                                dot_token_idx: TokenIdx(
                                                    280,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        281,
                                                    ),
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 38,
                                                dot_token_idx: TokenIdx(
                                                    282,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `ilen`,
                                                    token_idx: TokenIdx(
                                                        283,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    284,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    39..39,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    285,
                                                ),
                                            },
                                            Expr::Literal(
                                                TokenIdx(
                                                    287,
                                                ),
                                            ),
                                            Expr::BinaryOpn {
                                                lopd: 39,
                                                opr: PureClosed(
                                                    Sub,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    286,
                                                ),
                                                ropd: 40,
                                            },
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    36,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    278,
                                                ),
                                                items: ArenaIdxRange(
                                                    41..42,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    288,
                                                ),
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    292,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 43,
                                                dot_token_idx: TokenIdx(
                                                    293,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        294,
                                                    ),
                                                },
                                            },
                                            Expr::Literal(
                                                TokenIdx(
                                                    296,
                                                ),
                                            ),
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    44,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    295,
                                                ),
                                                items: ArenaIdxRange(
                                                    45..46,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    297,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `a`,
                                                token_idx: TokenIdx(
                                                    301,
                                                ),
                                                current_symbol_idx: 4,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `b`,
                                                token_idx: TokenIdx(
                                                    305,
                                                ),
                                                current_symbol_idx: 5,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 47,
                                                dot_token_idx: TokenIdx(
                                                    302,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        303,
                                                    ),
                                                },
                                            },
                                            Expr::Field {
                                                owner: 48,
                                                dot_token_idx: TokenIdx(
                                                    306,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        307,
                                                    ),
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 49,
                                                opr: PureClosed(
                                                    Sub,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    304,
                                                ),
                                                ropd: 50,
                                            },
                                            Expr::Bracketed {
                                                lpar_token_idx: TokenIdx(
                                                    300,
                                                ),
                                                item: 51,
                                                rpar_token_idx: TokenIdx(
                                                    308,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `a`,
                                                token_idx: TokenIdx(
                                                    315,
                                                ),
                                                current_symbol_idx: 4,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `b`,
                                                token_idx: TokenIdx(
                                                    319,
                                                ),
                                                current_symbol_idx: 5,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            Expr::Field {
                                                owner: 53,
                                                dot_token_idx: TokenIdx(
                                                    316,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        317,
                                                    ),
                                                },
                                            },
                                            Expr::Field {
                                                owner: 54,
                                                dot_token_idx: TokenIdx(
                                                    320,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        321,
                                                    ),
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 55,
                                                opr: PureClosed(
                                                    Sub,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    318,
                                                ),
                                                ropd: 56,
                                            },
                                            Expr::Bracketed {
                                                lpar_token_idx: TokenIdx(
                                                    314,
                                                ),
                                                item: 57,
                                                rpar_token_idx: TokenIdx(
                                                    322,
                                                ),
                                            },
                                            Expr::MethodCall {
                                                self_expr: 52,
                                                dot_token_idx: TokenIdx(
                                                    309,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `abs`,
                                                    token_idx: TokenIdx(
                                                        310,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    311,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    53..53,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    312,
                                                ),
                                            },
                                            Expr::MethodCall {
                                                self_expr: 58,
                                                dot_token_idx: TokenIdx(
                                                    323,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `abs`,
                                                    token_idx: TokenIdx(
                                                        324,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    325,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    59..59,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    326,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `contour_len`,
                                                token_idx: TokenIdx(
                                                    298,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 59,
                                                opr: PureClosed(
                                                    Add,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    313,
                                                ),
                                                ropd: 60,
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 61,
                                                opr: Assign(
                                                    Some(
                                                        Add,
                                                    ),
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    299,
                                                ),
                                                ropd: 62,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `contour_len`,
                                                token_idx: TokenIdx(
                                                    328,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::Block {
                                                stmts: ArenaIdxRange(
                                                    3..9,
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
                                                        223,
                                                    ),
                                                },
                                                let_variable_pattern: Ok(
                                                    LetVariablesPattern {
                                                        pattern: 1,
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
                                                            225,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    13,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        234,
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
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            236,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    17,
                                                ),
                                            },
                                            Stmt::Eval {
                                                expr_idx: 34,
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        205,
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
                                                            208,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    0,
                                                ),
                                            },
                                            Stmt::ForBetween {
                                                for_token: ForToken {
                                                    token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                },
                                                particulars: ForBetweenParticulars {
                                                    frame_var_token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                    frame_var_expr_idx: 2,
                                                    frame_var_ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 157,
                                                            },
                                                        ),
                                                    ),
                                                    range: ForBetweenRange {
                                                        initial_boundary: LoopBoundary {
                                                            bound_expr: Some(
                                                                1,
                                                            ),
                                                            kind: LowerOpen,
                                                        },
                                                        final_boundary: LoopBoundary {
                                                            bound_expr: Some(
                                                                6,
                                                            ),
                                                            kind: UpperOpen,
                                                        },
                                                        step: Constant(
                                                            1,
                                                        ),
                                                    },
                                                },
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            222,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..3,
                                                    ),
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        272,
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
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            274,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    42,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        289,
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
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            291,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    46,
                                                ),
                                            },
                                            Stmt::Eval {
                                                expr_idx: 63,
                                            },
                                            Stmt::Return {
                                                return_token: ReturnToken {
                                                    token_idx: TokenIdx(
                                                        327,
                                                    ),
                                                },
                                                result: Ok(
                                                    64,
                                                ),
                                            },
                                        ],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `contour_len`,
                                                        token_idx: TokenIdx(
                                                            207,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `a`,
                                                        token_idx: TokenIdx(
                                                            224,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `b`,
                                                        token_idx: TokenIdx(
                                                            235,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `a`,
                                                        token_idx: TokenIdx(
                                                            273,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `b`,
                                                        token_idx: TokenIdx(
                                                            290,
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
                                            Let,
                                            Let,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 148,
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
                                                                value: 178,
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
                                                                value: 208,
                                                            },
                                                        ),
                                                    ),
                                                    2,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 178,
                                                            },
                                                        ),
                                                    ),
                                                    3,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 208,
                                                            },
                                                        ),
                                                    ),
                                                    4,
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
                                                    ident: `contour_len`,
                                                    access_start: TokenIdx(
                                                        208,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                329,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `i`,
                                                    access_start: TokenIdx(
                                                        223,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                272,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::FrameVariable(
                                                        2,
                                                    ),
                                                },
                                                CurrentSymbol {
                                                    ident: `a`,
                                                    access_start: TokenIdx(
                                                        225,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                272,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `b`,
                                                    access_start: TokenIdx(
                                                        236,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                272,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `a`,
                                                    access_start: TokenIdx(
                                                        274,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                329,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `b`,
                                                    access_start: TokenIdx(
                                                        291,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                329,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 4,
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
                                            expr: 65,
                                        },
                                    ],
                                },
                            },
                            body: Ok(
                                65,
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
                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ident: `displacement`,
                                    ty_item_kind: Method,
                                },
                            ),
                            decl: TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ident: `displacement`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `displacement`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::raw_contour`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            },
                                        },
                                        ast_idx: 200,
                                        body: ArenaIdxRange(
                                            27..34,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        },
                                    },
                                    ast_idx: 31,
                                    ident: `displacement`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::raw_contour`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ident: `displacement`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 31,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                },
                                                            },
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
                                                                    43,
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
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        TypePath(`core::num::i32`, `Alien`),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::num::i32`, `Alien`),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
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
                                                        334,
                                                    ),
                                                    ident: `i32`,
                                                    entity_path: TypePath(`core::num::i32`, `Alien`),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        338,
                                                    ),
                                                    ident: `i32`,
                                                    entity_path: TypePath(`core::num::i32`, `Alien`),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        341,
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
                                                data: [
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                332,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                336,
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
                                                                    value: 210,
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
                                                                    value: 211,
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
                                                        ident: `start`,
                                                        access_start: TokenIdx(
                                                            333,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        ident: `end`,
                                                        access_start: TokenIdx(
                                                            337,
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
                                            331,
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
                                                    333,
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
                                                    337,
                                                ),
                                            },
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken {
                                            token_idx: TokenIdx(
                                                335,
                                            ),
                                        },
                                    ],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            339,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            340,
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
                                            342,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        impl_block_kind: ImplBlockKind::Type {
                                                                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        },
                                                                    },
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
                                                                            43,
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                TypePath(`core::num::i32`, `Alien`),
                                                            ),
                                                        },
                                                        Expr::EntityPath {
                                                            entity_path_expr: 1,
                                                            entity_path: Some(
                                                                TypePath(`core::num::i32`, `Alien`),
                                                            ),
                                                        },
                                                        Expr::EntityPath {
                                                            entity_path_expr: 2,
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
                                                                334,
                                                            ),
                                                            ident: `i32`,
                                                            entity_path: TypePath(`core::num::i32`, `Alien`),
                                                        },
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                338,
                                                            ),
                                                            ident: `i32`,
                                                            entity_path: TypePath(`core::num::i32`, `Alien`),
                                                        },
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                341,
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
                                                        data: [
                                                            PatternExpr::Identifier {
                                                                ident_token: IdentifierToken {
                                                                    ident: `start`,
                                                                    token_idx: TokenIdx(
                                                                        332,
                                                                    ),
                                                                },
                                                                liason: None,
                                                            },
                                                            PatternExpr::Identifier {
                                                                ident_token: IdentifierToken {
                                                                    ident: `end`,
                                                                    token_idx: TokenIdx(
                                                                        336,
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
                                                                            value: 210,
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
                                                                            value: 211,
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
                                                                ident: `start`,
                                                                access_start: TokenIdx(
                                                                    333,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::RegularParameter {
                                                                    pattern_symbol_idx: 0,
                                                                },
                                                            },
                                                            CurrentSymbol {
                                                                ident: `end`,
                                                                access_start: TokenIdx(
                                                                    337,
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
                                                        kind: OutputType,
                                                        expr: 2,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        DefnExprPath::AssociatedItem(
                                            AssociatedItemId {
                                                impl_block_id: ImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                    346,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 0,
                                                dot_token_idx: TokenIdx(
                                                    347,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        348,
                                                    ),
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 1,
                                                dot_token_idx: TokenIdx(
                                                    349,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `ilen`,
                                                    token_idx: TokenIdx(
                                                        350,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    351,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    2..2,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    352,
                                                ),
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    356,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 3,
                                                dot_token_idx: TokenIdx(
                                                    357,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        358,
                                                    ),
                                                },
                                            },
                                            Expr::InheritedSymbol {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    360,
                                                ),
                                                inherited_symbol_idx: 0,
                                                inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `N`,
                                                token_idx: TokenIdx(
                                                    362,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 5,
                                                opr: PureClosed(
                                                    RemEuclid,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    361,
                                                ),
                                                ropd: 6,
                                            },
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    4,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    359,
                                                ),
                                                items: ArenaIdxRange(
                                                    7..8,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    363,
                                                ),
                                            },
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    367,
                                                ),
                                            ),
                                            Expr::Field {
                                                owner: 9,
                                                dot_token_idx: TokenIdx(
                                                    368,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `points`,
                                                    token_idx: TokenIdx(
                                                        369,
                                                    ),
                                                },
                                            },
                                            Expr::InheritedSymbol {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    371,
                                                ),
                                                inherited_symbol_idx: 1,
                                                inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `N`,
                                                token_idx: TokenIdx(
                                                    373,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 11,
                                                opr: PureClosed(
                                                    RemEuclid,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    372,
                                                ),
                                                ropd: 12,
                                            },
                                            Expr::NewBoxList {
                                                caller: Some(
                                                    10,
                                                ),
                                                lbox_token_idx: TokenIdx(
                                                    370,
                                                ),
                                                items: ArenaIdxRange(
                                                    13..14,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    374,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ct_start`,
                                                token_idx: TokenIdx(
                                                    375,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ct_end`,
                                                token_idx: TokenIdx(
                                                    379,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 15,
                                                dot_token_idx: TokenIdx(
                                                    376,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `to`,
                                                    token_idx: TokenIdx(
                                                        377,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    378,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    16..17,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    380,
                                                ),
                                            },
                                            Expr::Block {
                                                stmts: ArenaIdxRange(
                                                    0..4,
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
                                                        343,
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
                                                            345,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    2,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        353,
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
                                                            355,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    8,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        364,
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
                                                            366,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    14,
                                                ),
                                            },
                                            Stmt::Eval {
                                                expr_idx: 17,
                                            },
                                        ],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `N`,
                                                        token_idx: TokenIdx(
                                                            344,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `ct_start`,
                                                        token_idx: TokenIdx(
                                                            354,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `ct_end`,
                                                        token_idx: TokenIdx(
                                                            365,
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
                                                                value: 213,
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
                                                                value: 214,
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
                                                                value: 215,
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
                                                    ident: `start`,
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    kind: InheritedSymbolKind::RegularParameter,
                                                },
                                                InheritedSymbol {
                                                    ident: `end`,
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    kind: InheritedSymbolKind::RegularParameter,
                                                },
                                            ],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: `N`,
                                                    access_start: TokenIdx(
                                                        345,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                381,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `ct_start`,
                                                    access_start: TokenIdx(
                                                        355,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                381,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `ct_end`,
                                                    access_start: TokenIdx(
                                                        366,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                381,
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
                                            expr: 18,
                                        },
                                    ],
                                },
                            },
                            body: Ok(
                                18,
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)