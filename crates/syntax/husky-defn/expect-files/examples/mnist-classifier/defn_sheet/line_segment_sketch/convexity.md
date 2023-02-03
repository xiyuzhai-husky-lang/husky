Ok(
    DefnSheet {
        defns: [
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                            ast_idx: 22,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
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
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::basic::bool`, `Alien`),
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
                                                ident: `LineSegmentSketch`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    28,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    31,
                                                ),
                                                ident: `bool`,
                                                entity_path: TypePath(`core::basic::bool`, `Alien`),
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
                                                        ident: `line_segment_sketch`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `index`,
                                                        token_idx: TokenIdx(
                                                            26,
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
                                                                value: 98,
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
                                                                value: 336,
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
                                                    ident: `line_segment_sketch`,
                                                    access_start: TokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `index`,
                                                    access_start: TokenIdx(
                                                        27,
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
                                        21,
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
                                                23,
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
                                                27,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        29,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        30,
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
                                        32,
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
                                                    FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
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
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::i32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            TypePath(`core::basic::bool`, `Alien`),
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
                                                        ident: `LineSegmentSketch`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            28,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: TypePath(`core::num::i32`, `Alien`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                        ident: `bool`,
                                                        entity_path: TypePath(`core::basic::bool`, `Alien`),
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
                                                                ident: `line_segment_sketch`,
                                                                token_idx: TokenIdx(
                                                                    22,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `index`,
                                                                token_idx: TokenIdx(
                                                                    26,
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
                                                                        value: 98,
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
                                                                        value: 336,
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
                                                            ident: `line_segment_sketch`,
                                                            access_start: TokenIdx(
                                                                23,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `index`,
                                                            access_start: TokenIdx(
                                                                27,
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
                                        FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                36,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                37,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    38,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 1,
                                            dot_token_idx: TokenIdx(
                                                39,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    40,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                41,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..2,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                42,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                46,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            this_expr: 3,
                                            dot_token_idx: TokenIdx(
                                                47,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    48,
                                                ),
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                50,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                52,
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
                                                51,
                                            ),
                                            ropd: 6,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                4,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                49,
                                            ),
                                            items: ArenaIdxRange(
                                                7..8,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                53,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            this_expr: 8,
                                            dot_token_idx: TokenIdx(
                                                54,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    55,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                56,
                                            ),
                                            arguments: ArenaIdxRange(
                                                9..9,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                57,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                61,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            this_expr: 10,
                                            dot_token_idx: TokenIdx(
                                                62,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    63,
                                                ),
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                66,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                68,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 12,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                67,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                65,
                                            ),
                                            item: 14,
                                            rpar_token_idx: TokenIdx(
                                                69,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                71,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 15,
                                            opr: PureClosed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                70,
                                            ),
                                            ropd: 16,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                11,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                64,
                                            ),
                                            items: ArenaIdxRange(
                                                17..18,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                72,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            this_expr: 18,
                                            dot_token_idx: TokenIdx(
                                                73,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    74,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                75,
                                            ),
                                            arguments: ArenaIdxRange(
                                                19..19,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                76,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_displacement`,
                                            token_idx: TokenIdx(
                                                80,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                84,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 20,
                                            dot_token_idx: TokenIdx(
                                                81,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `rotation_direction_to`,
                                                token_idx: TokenIdx(
                                                    82,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                83,
                                            ),
                                            arguments: ArenaIdxRange(
                                                21..22,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                85,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `is_rotation_counterclockwise_result`,
                                            token_idx: TokenIdx(
                                                87,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                89,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 23,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                88,
                                            ),
                                            ropd: 24,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                96,
                                            ),
                                        ),
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                95,
                                            ),
                                            opd: 26,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                100,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            this_expr: 28,
                                            dot_token_idx: TokenIdx(
                                                101,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    102,
                                                ),
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                105,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                107,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 30,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                106,
                                            ),
                                            ropd: 31,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                104,
                                            ),
                                            item: 32,
                                            rpar_token_idx: TokenIdx(
                                                108,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                110,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 33,
                                            opr: PureClosed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                109,
                                            ),
                                            ropd: 34,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                29,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                103,
                                            ),
                                            items: ArenaIdxRange(
                                                35..36,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                111,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 36,
                                            dot_token_idx: TokenIdx(
                                                112,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    113,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                115,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 38,
                                            dot_token_idx: TokenIdx(
                                                116,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    117,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident: `i1`,
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                40,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                121,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 39,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                118,
                                            ),
                                            ropd: 40,
                                        },
                                        Expr::Field {
                                            this_expr: 41,
                                            dot_token_idx: TokenIdx(
                                                122,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    123,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 42,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                120,
                                            ),
                                            ropd: 43,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                128,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            this_expr: 45,
                                            dot_token_idx: TokenIdx(
                                                129,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `contour`,
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                134,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 47,
                                            dot_token_idx: TokenIdx(
                                                135,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    136,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i1`,
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                40,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            this_expr: 46,
                                            dot_token_idx: TokenIdx(
                                                131,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    132,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                133,
                                            ),
                                            arguments: ArenaIdxRange(
                                                48..50,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                139,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                142,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                146,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `displacement`,
                                            token_idx: TokenIdx(
                                                150,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 52,
                                            dot_token_idx: TokenIdx(
                                                147,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `cross`,
                                                token_idx: TokenIdx(
                                                    148,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                149,
                                            ),
                                            arguments: ArenaIdxRange(
                                                53..54,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                151,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                140,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 51,
                                            dot_token_idx: TokenIdx(
                                                143,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    144,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                145,
                                            ),
                                            arguments: ArenaIdxRange(
                                                54..55,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                152,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 55,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                141,
                                            ),
                                            ropd: 56,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                158,
                                            ),
                                        ),
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                157,
                                            ),
                                            opd: 58,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                162,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            this_expr: 60,
                                            dot_token_idx: TokenIdx(
                                                163,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    164,
                                                ),
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `index`,
                                            token_idx: TokenIdx(
                                                166,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                168,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 62,
                                            opr: PureClosed(
                                                RemEuclid,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                167,
                                            ),
                                            ropd: 63,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                61,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                165,
                                            ),
                                            items: ArenaIdxRange(
                                                64..65,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                169,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 65,
                                            dot_token_idx: TokenIdx(
                                                170,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    171,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_interval`,
                                            token_idx: TokenIdx(
                                                173,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 67,
                                            dot_token_idx: TokenIdx(
                                                174,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    175,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                177,
                                            ),
                                            ident: `i2`,
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                69,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_interval`,
                                            token_idx: TokenIdx(
                                                179,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 68,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                176,
                                            ),
                                            ropd: 69,
                                        },
                                        Expr::Field {
                                            this_expr: 70,
                                            dot_token_idx: TokenIdx(
                                                180,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    181,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 71,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ropd: 72,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                186,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            this_expr: 74,
                                            dot_token_idx: TokenIdx(
                                                187,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `contour`,
                                                token_idx: TokenIdx(
                                                    188,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_interval`,
                                            token_idx: TokenIdx(
                                                192,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 76,
                                            dot_token_idx: TokenIdx(
                                                193,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    194,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i2`,
                                            token_idx: TokenIdx(
                                                196,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                69,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            this_expr: 75,
                                            dot_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    190,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                191,
                                            ),
                                            arguments: ArenaIdxRange(
                                                77..79,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                197,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                200,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_displacement`,
                                            token_idx: TokenIdx(
                                                204,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `displacement`,
                                            token_idx: TokenIdx(
                                                208,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 81,
                                            dot_token_idx: TokenIdx(
                                                205,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `cross`,
                                                token_idx: TokenIdx(
                                                    206,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                207,
                                            ),
                                            arguments: ArenaIdxRange(
                                                82..83,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                209,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                198,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 80,
                                            dot_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    202,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                203,
                                            ),
                                            arguments: ArenaIdxRange(
                                                83..84,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 84,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                199,
                                            ),
                                            ropd: 85,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `current_raw_cross`,
                                            token_idx: TokenIdx(
                                                212,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `previous_raw_cross`,
                                            token_idx: TokenIdx(
                                                214,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 87,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                213,
                                            ),
                                            ropd: 88,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `is_rotation_counterclockwise_result`,
                                            token_idx: TokenIdx(
                                                218,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                220,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 90,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                219,
                                            ),
                                            ropd: 91,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                12..17,
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
                                                    125,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        127,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                50,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 57,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    183,
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
                                                        185,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                79,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 86,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    91,
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
                                                        94,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                27,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    97,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 5,
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
                                                        99,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                37,
                                            ),
                                        },
                                        Stmt::ForBetween {
                                            for_token: ForToken {
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    119,
                                                ),
                                                frame_var_expr_idx: 40,
                                                frame_var_ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 158,
                                                        },
                                                    ),
                                                ),
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            39,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            43,
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
                                                        124,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..2,
                                                ),
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    153,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                59,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    159,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                66,
                                            ),
                                        },
                                        Stmt::ForBetween {
                                            for_token: ForToken {
                                                token_idx: TokenIdx(
                                                    172,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    177,
                                                ),
                                                frame_var_expr_idx: 69,
                                                frame_var_ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 160,
                                                        },
                                                    ),
                                                ),
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            68,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            72,
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
                                                        182,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    2..4,
                                                ),
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    211,
                                                ),
                                            },
                                            result: Ok(
                                                89,
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    217,
                                                ),
                                            },
                                            result: Ok(
                                                92,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    33,
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
                                                        35,
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
                                                    43,
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
                                                        45,
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
                                                    58,
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
                                                        60,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                19,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    77,
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
                                                        79,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                22,
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        86,
                                                    ),
                                                },
                                                condition: Ok(
                                                    25,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        4..11,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                ElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            215,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                216,
                                                            ),
                                                        },
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            11..12,
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
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `current_displacement`,
                                                    token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `previous_displacement`,
                                                    token_idx: TokenIdx(
                                                        59,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `is_rotation_counterclockwise_result`,
                                                    token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `previous_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        93,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `previous_interval`,
                                                    token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `current_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `current_interval`,
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        184,
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
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 295,
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
                                                            value: 337,
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
                                                            value: 338,
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
                                                            value: 339,
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
                                                            value: 340,
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
                                                            value: 341,
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
                                                            value: 198,
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
                                                            value: 342,
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
                                                            value: 343,
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
                                                            value: 198,
                                                        },
                                                    ),
                                                ),
                                                9,
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
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                ident: `line_segment_sketch`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            InheritedSymbol {
                                                ident: `index`,
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
                                                ident: `L`,
                                                access_start: TokenIdx(
                                                    35,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            221,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `current_displacement`,
                                                access_start: TokenIdx(
                                                    45,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            221,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `previous_displacement`,
                                                access_start: TokenIdx(
                                                    60,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            221,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `is_rotation_counterclockwise_result`,
                                                access_start: TokenIdx(
                                                    79,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            221,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `previous_raw_cross`,
                                                access_start: TokenIdx(
                                                    94,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `previous_interval`,
                                                access_start: TokenIdx(
                                                    99,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `i1`,
                                                access_start: TokenIdx(
                                                    125,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            153,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable(
                                                    40,
                                                ),
                                            },
                                            CurrentSymbol {
                                                ident: `displacement`,
                                                access_start: TokenIdx(
                                                    127,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            153,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `current_raw_cross`,
                                                access_start: TokenIdx(
                                                    156,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `current_interval`,
                                                access_start: TokenIdx(
                                                    161,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `i2`,
                                                access_start: TokenIdx(
                                                    183,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            211,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable(
                                                    69,
                                                ),
                                            },
                                            CurrentSymbol {
                                                ident: `displacement`,
                                                access_start: TokenIdx(
                                                    185,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            211,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 9,
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
                                        expr: 93,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            93,
                        ),
                    },
                ),
            ),
        ],
    },
)