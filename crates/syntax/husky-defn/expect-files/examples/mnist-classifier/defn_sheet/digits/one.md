Ok(
    DefnSheet {
        defns: [
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                            ast_idx: 68,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        60,
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
                                        62,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        61,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 346,
                                                            },
                                                        ),
                                                    ),
                                                },
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
                                        allow_self_type: False,
                                        allow_self_value: False,
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
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                67,
                                            ),
                                            items: ArenaIdxRange(
                                                1..4,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                73,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                64,
                                            ),
                                            arguments: ArenaIdxRange(
                                                4..6,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                74,
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
                                                63,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                65,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                68,
                                            ),
                                            ident: `downmost`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                70,
                                            ),
                                            ident: `upmost`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                72,
                                            ),
                                            ident: `hat`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr: 6,
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
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 7,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            7,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                            ast_idx: 69,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        78,
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
                                        81,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 106,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            Expr::PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    79,
                                                ),
                                                opd: 0,
                                            },
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
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 1,
                                        },
                                    ],
                                },
                            },
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    82,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    84,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    86,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 1,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                85,
                                            ),
                                            ropd: 2,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                88,
                                            ),
                                        ),
                                        Expr::Field {
                                            this_expr: 3,
                                            dot_token_idx: TokenIdx(
                                                91,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `max_hole_ilen`,
                                                token_idx: TokenIdx(
                                                    92,
                                                ),
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                83,
                                            ),
                                            arguments: ArenaIdxRange(
                                                4..8,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                95,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 8,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                96,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                104,
                                            ),
                                            items: ArenaIdxRange(
                                                11..11,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                105,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 10,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                101,
                                            ),
                                            arguments: ArenaIdxRange(
                                                11..13,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                106,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `simp_one_match`,
                                            token_idx: TokenIdx(
                                                108,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 14,
                                            dot_token_idx: TokenIdx(
                                                109,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    110,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                112,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 15,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                111,
                                            ),
                                            ropd: 16,
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    116,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 19,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                117,
                                            ),
                                            ropd: 20,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                120,
                                            ),
                                        ),
                                        Expr::Field {
                                            this_expr: 21,
                                            dot_token_idx: TokenIdx(
                                                123,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `max_row_span`,
                                                token_idx: TokenIdx(
                                                    124,
                                                ),
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 18,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                115,
                                            ),
                                            arguments: ArenaIdxRange(
                                                22..25,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                125,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 25,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                126,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 5,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 27,
                                            dot_token_idx: TokenIdx(
                                                129,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `max_row_span`,
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                132,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 28,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                131,
                                            ),
                                            ropd: 29,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 6,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 31,
                                            dot_token_idx: TokenIdx(
                                                136,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `max_hole_ilen`,
                                                token_idx: TokenIdx(
                                                    137,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                139,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 32,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                138,
                                            ),
                                            ropd: 33,
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    140,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 35,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                141,
                                            ),
                                            ropd: 36,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 7,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 38,
                                            dot_token_idx: TokenIdx(
                                                147,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `max_hole_ilen`,
                                                token_idx: TokenIdx(
                                                    148,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                150,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 39,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                149,
                                            ),
                                            ropd: 40,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 8,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                154,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 42,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                153,
                                            ),
                                            ropd: 43,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 9,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 45,
                                            dot_token_idx: TokenIdx(
                                                159,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    160,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                162,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                46,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                161,
                                            ),
                                            items: ArenaIdxRange(
                                                47..48,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                163,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 10,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 49,
                                            dot_token_idx: TokenIdx(
                                                168,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    169,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                171,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                50,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                170,
                                            ),
                                            items: ArenaIdxRange(
                                                51..52,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                172,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 11,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 53,
                                            dot_token_idx: TokenIdx(
                                                177,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    178,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                180,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                54,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                179,
                                            ),
                                            items: ArenaIdxRange(
                                                55..56,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                181,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                183,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Expr::Be {
                                            src: 57,
                                            be_token_idx: TokenIdx(
                                                184,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 4,
                                                },
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `simp_one_match`,
                                            token_idx: TokenIdx(
                                                188,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 59,
                                            dot_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    190,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                192,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 60,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                191,
                                            ),
                                            ropd: 61,
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    193,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    195,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    197,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `simp_one_match`,
                                            token_idx: TokenIdx(
                                                201,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 66,
                                            dot_token_idx: TokenIdx(
                                                202,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `angle_change_norm`,
                                                token_idx: TokenIdx(
                                                    203,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 64,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                196,
                                            ),
                                            ropd: 65,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                199,
                                            ),
                                        ),
                                        Expr::MethodCall {
                                            this_expr: 67,
                                            dot_token_idx: TokenIdx(
                                                204,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    205,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                206,
                                            ),
                                            arguments: ArenaIdxRange(
                                                68..68,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                207,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 63,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                194,
                                            ),
                                            arguments: ArenaIdxRange(
                                                68..71,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                208,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 71,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                209,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 12,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                211,
                                            ),
                                        ),
                                        Expr::Field {
                                            this_expr: 73,
                                            dot_token_idx: TokenIdx(
                                                214,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    215,
                                                ),
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 13,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 74,
                                            opr: PureClosed(
                                                Mul,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                212,
                                            ),
                                            ropd: 75,
                                        },
                                        Expr::Field {
                                            this_expr: 76,
                                            dot_token_idx: TokenIdx(
                                                218,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    219,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 77,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                216,
                                            ),
                                            ropd: 78,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                221,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 79,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                220,
                                            ),
                                            ropd: 80,
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    222,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    224,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 82,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                223,
                                            ),
                                            ropd: 83,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                228,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 85,
                                            dot_token_idx: TokenIdx(
                                                229,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    230,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                234,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 87,
                                            dot_token_idx: TokenIdx(
                                                235,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    236,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 86,
                                            dot_token_idx: TokenIdx(
                                                231,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    232,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 88,
                                            dot_token_idx: TokenIdx(
                                                237,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    238,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 89,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                233,
                                            ),
                                            ropd: 90,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 14,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 92,
                                            dot_token_idx: TokenIdx(
                                                241,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    242,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                244,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 93,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                243,
                                            ),
                                            ropd: 94,
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    245,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    247,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    249,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 15,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                257,
                                            ),
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 16,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 17,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 18,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 97,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                248,
                                            ),
                                            ropd: 98,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                251,
                                            ),
                                        ),
                                        Expr::MethodCall {
                                            this_expr: 99,
                                            dot_token_idx: TokenIdx(
                                                254,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `top_k_row_span_sum`,
                                                token_idx: TokenIdx(
                                                    255,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                256,
                                            ),
                                            arguments: ArenaIdxRange(
                                                100..101,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                258,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 101,
                                            dot_token_idx: TokenIdx(
                                                261,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    262,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 102,
                                            dot_token_idx: TokenIdx(
                                                265,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    266,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 103,
                                            dot_token_idx: TokenIdx(
                                                269,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `angle_change_norm`,
                                                token_idx: TokenIdx(
                                                    270,
                                                ),
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 96,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                246,
                                            ),
                                            arguments: ArenaIdxRange(
                                                104..110,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                272,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 110,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                273,
                                            ),
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    274,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    276,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    278,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 19,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 115,
                                            dot_token_idx: TokenIdx(
                                                283,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    284,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                286,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                116,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                285,
                                            ),
                                            items: ArenaIdxRange(
                                                117..118,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                287,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 20,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 119,
                                            dot_token_idx: TokenIdx(
                                                292,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    293,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                295,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                120,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                294,
                                            ),
                                            items: ArenaIdxRange(
                                                121..122,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                296,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 122,
                                            dot_token_idx: TokenIdx(
                                                297,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    298,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 113,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                277,
                                            ),
                                            ropd: 114,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                280,
                                            ),
                                        ),
                                        Expr::Field {
                                            this_expr: 118,
                                            dot_token_idx: TokenIdx(
                                                288,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    289,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 123,
                                            dot_token_idx: TokenIdx(
                                                299,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    300,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                301,
                                            ),
                                            arguments: ArenaIdxRange(
                                                124..124,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                302,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 112,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                275,
                                            ),
                                            arguments: ArenaIdxRange(
                                                124..128,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                304,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 128,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                305,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                307,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Expr::Be {
                                            src: 130,
                                            be_token_idx: TokenIdx(
                                                308,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 6,
                                                },
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                312,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 132,
                                            dot_token_idx: TokenIdx(
                                                313,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    314,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                318,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 134,
                                            dot_token_idx: TokenIdx(
                                                319,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    320,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 133,
                                            dot_token_idx: TokenIdx(
                                                315,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    316,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 135,
                                            dot_token_idx: TokenIdx(
                                                321,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    322,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 136,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                317,
                                            ),
                                            ropd: 137,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                324,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 138,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                323,
                                            ),
                                            ropd: 139,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                328,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 141,
                                            dot_token_idx: TokenIdx(
                                                329,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    330,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                332,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 143,
                                            dot_token_idx: TokenIdx(
                                                333,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    334,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 144,
                                            dot_token_idx: TokenIdx(
                                                335,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    336,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                338,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 145,
                                            opr: PureClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                337,
                                            ),
                                            ropd: 146,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                142,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                331,
                                            ),
                                            items: ArenaIdxRange(
                                                147..148,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                339,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `long_vertical`,
                                            token_idx: TokenIdx(
                                                343,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 1,
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 149,
                                            dot_token_idx: TokenIdx(
                                                344,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    345,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                346,
                                            ),
                                            arguments: ArenaIdxRange(
                                                150..150,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                347,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                349,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 151,
                                            dot_token_idx: TokenIdx(
                                                350,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    351,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                353,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 152,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                352,
                                            ),
                                            ropd: 153,
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    354,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    356,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    358,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                362,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 5,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                366,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 5,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost`,
                                            token_idx: TokenIdx(
                                                370,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 5,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 160,
                                            dot_token_idx: TokenIdx(
                                                371,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    372,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 156,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                357,
                                            ),
                                            ropd: 157,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                360,
                                            ),
                                        ),
                                        Expr::Field {
                                            this_expr: 158,
                                            dot_token_idx: TokenIdx(
                                                363,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    364,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 159,
                                            dot_token_idx: TokenIdx(
                                                367,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    368,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 161,
                                            dot_token_idx: TokenIdx(
                                                373,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    374,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                375,
                                            ),
                                            arguments: ArenaIdxRange(
                                                162..162,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                376,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 155,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                355,
                                            ),
                                            arguments: ArenaIdxRange(
                                                162..167,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                377,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 167,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                378,
                                            ),
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    379,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    381,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    383,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                387,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                393,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `long_vertical_dp`,
                                            token_idx: TokenIdx(
                                                397,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 173,
                                            dot_token_idx: TokenIdx(
                                                394,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    395,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 174,
                                            dot_token_idx: TokenIdx(
                                                398,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    399,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 170,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                382,
                                            ),
                                            ropd: 171,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                385,
                                            ),
                                        ),
                                        Expr::MethodCall {
                                            this_expr: 172,
                                            dot_token_idx: TokenIdx(
                                                388,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    389,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                390,
                                            ),
                                            arguments: ArenaIdxRange(
                                                173..173,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                391,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 175,
                                            opr: PureClosed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                396,
                                            ),
                                            ropd: 176,
                                        },
                                        Expr::FunctionCall {
                                            function: 169,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                380,
                                            ),
                                            arguments: ArenaIdxRange(
                                                177..181,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                400,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 181,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                401,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `hat`,
                                            token_idx: TokenIdx(
                                                403,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Expr::Be {
                                            src: 183,
                                            be_token_idx: TokenIdx(
                                                404,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 9,
                                                },
                                            ),
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    407,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    409,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    411,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 21,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 188,
                                            dot_token_idx: TokenIdx(
                                                416,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    417,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                419,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                189,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                418,
                                            ),
                                            items: ArenaIdxRange(
                                                190..191,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                420,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 22,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 192,
                                            dot_token_idx: TokenIdx(
                                                425,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    426,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                428,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                193,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                427,
                                            ),
                                            items: ArenaIdxRange(
                                                194..195,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                429,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 23,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 196,
                                            dot_token_idx: TokenIdx(
                                                434,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    435,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                437,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                197,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                436,
                                            ),
                                            items: ArenaIdxRange(
                                                198..199,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                438,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 199,
                                            dot_token_idx: TokenIdx(
                                                439,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    440,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 186,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                410,
                                            ),
                                            ropd: 187,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                413,
                                            ),
                                        ),
                                        Expr::Field {
                                            this_expr: 191,
                                            dot_token_idx: TokenIdx(
                                                421,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    422,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 195,
                                            dot_token_idx: TokenIdx(
                                                430,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    431,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 200,
                                            dot_token_idx: TokenIdx(
                                                441,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    442,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                443,
                                            ),
                                            arguments: ArenaIdxRange(
                                                201..201,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                444,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 185,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                408,
                                            ),
                                            arguments: ArenaIdxRange(
                                                201..206,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                446,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 206,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                447,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost_number_of_strokes`,
                                            token_idx: TokenIdx(
                                                449,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                451,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 208,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                450,
                                            ),
                                            ropd: 209,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                455,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 8,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 211,
                                            dot_token_idx: TokenIdx(
                                                456,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    457,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                459,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 8,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 213,
                                            dot_token_idx: TokenIdx(
                                                460,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    461,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 214,
                                            dot_token_idx: TokenIdx(
                                                462,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    463,
                                                ),
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                212,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                458,
                                            ),
                                            items: ArenaIdxRange(
                                                215..216,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                464,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost_hat`,
                                            token_idx: TokenIdx(
                                                468,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 1,
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 217,
                                            dot_token_idx: TokenIdx(
                                                469,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    470,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                471,
                                            ),
                                            arguments: ArenaIdxRange(
                                                218..218,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                472,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                476,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 11,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 219,
                                            dot_token_idx: TokenIdx(
                                                477,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    478,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost`,
                                            token_idx: TokenIdx(
                                                480,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 11,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 221,
                                            dot_token_idx: TokenIdx(
                                                481,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    482,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 222,
                                            dot_token_idx: TokenIdx(
                                                483,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    484,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                486,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 223,
                                            opr: PureClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                485,
                                            ),
                                            ropd: 224,
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                220,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                479,
                                            ),
                                            items: ArenaIdxRange(
                                                225..226,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                487,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost_feet`,
                                            token_idx: TokenIdx(
                                                491,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 1,
                                            },
                                        },
                                        Expr::MethodCall {
                                            this_expr: 227,
                                            dot_token_idx: TokenIdx(
                                                492,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    493,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                494,
                                            ),
                                            arguments: ArenaIdxRange(
                                                228..228,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                495,
                                            ),
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    496,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    498,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    500,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `downmost_hat_dp`,
                                            token_idx: TokenIdx(
                                                504,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost_feet_dp`,
                                            token_idx: TokenIdx(
                                                508,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 230,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                499,
                                            ),
                                            ropd: 231,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                502,
                                            ),
                                        ),
                                        Expr::Field {
                                            this_expr: 232,
                                            dot_token_idx: TokenIdx(
                                                505,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    506,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 233,
                                            dot_token_idx: TokenIdx(
                                                509,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    510,
                                                ),
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 229,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                497,
                                            ),
                                            arguments: ArenaIdxRange(
                                                234..238,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                511,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 238,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                512,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downmost_number_of_strokes`,
                                            token_idx: TokenIdx(
                                                514,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 8,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                516,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 240,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                515,
                                            ),
                                            ropd: 241,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 24,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 243,
                                            dot_token_idx: TokenIdx(
                                                521,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    522,
                                                ),
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 25,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 245,
                                            dot_token_idx: TokenIdx(
                                                527,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    528,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                532,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `b`,
                                            token_idx: TokenIdx(
                                                534,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 1,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 247,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                533,
                                            ),
                                            ropd: 248,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                538,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                540,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 250,
                                            opr: PureClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                539,
                                            ),
                                            ropd: 251,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                545,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                542,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 1,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                544,
                                            ),
                                            opd: 253,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 254,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                543,
                                            ),
                                            ropd: 255,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                547,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 1,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                549,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 257,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                548,
                                            ),
                                            ropd: 258,
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    550,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    552,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 376,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 260,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                551,
                                            ),
                                            ropd: 261,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                41..44,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                90,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                94,
                                            ),
                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                            entity_path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                100,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                102,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                122,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                128,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                135,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                146,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                152,
                                            ),
                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                            entity_path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                158,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                167,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                176,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                213,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                217,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                240,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                253,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                260,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                264,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                268,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                282,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                291,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                415,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                424,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                433,
                                            ),
                                            ident: `one_fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                520,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                526,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                            condition: Ok(
                                                34,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 26,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        127,
                                                    ),
                                                },
                                                condition: Ok(
                                                    30,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            133,
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
                                            else_branch: None,
                                        },
                                        Stmt::Eval {
                                            expr: 37,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    187,
                                                ),
                                            },
                                            condition: Ok(
                                                62,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 72,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    210,
                                                ),
                                            },
                                            condition: Ok(
                                                81,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 84,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    311,
                                                ),
                                            },
                                            condition: Ok(
                                                140,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    325,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 7,
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
                                                        327,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                148,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    340,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 8,
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
                                                        342,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                150,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    348,
                                                ),
                                            },
                                            condition: Ok(
                                                154,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 168,
                                        },
                                        Stmt::Eval {
                                            expr: 182,
                                        },
                                        Stmt::Eval {
                                            expr: 207,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    145,
                                                ),
                                            },
                                            condition: Ok(
                                                41,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    151,
                                                ),
                                            },
                                            condition: Ok(
                                                44,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    155,
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
                                                        157,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                48,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    164,
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
                                                        166,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                52,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    173,
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
                                                        175,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                56,
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        182,
                                                    ),
                                                },
                                                condition: Ok(
                                                    58,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            186,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        4..8,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    225,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 5,
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
                                                        227,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                91,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    239,
                                                ),
                                            },
                                            condition: Ok(
                                                95,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 111,
                                        },
                                        Stmt::Eval {
                                            expr: 129,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        306,
                                                    ),
                                                },
                                                condition: Ok(
                                                    131,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            310,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        8..14,
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
                                                        402,
                                                    ),
                                                },
                                                condition: Ok(
                                                    184,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            406,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    448,
                                                ),
                                            },
                                            condition: Ok(
                                                210,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    452,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 10,
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
                                                        454,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                216,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    465,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 11,
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
                                                        467,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                218,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    473,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 12,
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
                                                        475,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                226,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    488,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 13,
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
                                                        490,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                228,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 239,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    513,
                                                ),
                                            },
                                            condition: Ok(
                                                242,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    517,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 14,
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
                                                        519,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                244,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    523,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 15,
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
                                                        525,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                246,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    529,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 16,
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
                                                        531,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                249,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    535,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 17,
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
                                                        537,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                252,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    541,
                                                ),
                                            },
                                            condition: Ok(
                                                256,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    546,
                                                ),
                                            },
                                            condition: Ok(
                                                259,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 262,
                                        },
                                        Stmt::Eval {
                                            expr: 9,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    97,
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
                                                        99,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                13,
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        107,
                                                    ),
                                                },
                                                condition: Ok(
                                                    17,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                ElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            143,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                144,
                                                            ),
                                                        },
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            15..41,
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
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `hat`,
                                                    token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `downmost_number_of_strokes`,
                                                    token_idx: TokenIdx(
                                                        226,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        309,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `long_vertical`,
                                                    token_idx: TokenIdx(
                                                        326,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        341,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        405,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `downmost_hat`,
                                                    token_idx: TokenIdx(
                                                        453,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `downmost_hat_dp`,
                                                    token_idx: TokenIdx(
                                                        466,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `downmost_feet`,
                                                    token_idx: TokenIdx(
                                                        474,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `downmost_feet_dp`,
                                                    token_idx: TokenIdx(
                                                        489,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        518,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `b`,
                                                    token_idx: TokenIdx(
                                                        524,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        530,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `d`,
                                                    token_idx: TokenIdx(
                                                        536,
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
                                        Be,
                                        Let,
                                        Be,
                                        Let,
                                        Let,
                                        Be,
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
                                                            value: 378,
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
                                                            value: 373,
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
                                                            value: 374,
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
                                                            value: 375,
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
                                                            value: 361,
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
                                                            value: 379,
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
                                                            value: 364,
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
                                                            value: 380,
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
                                                            value: 381,
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
                                                            value: 364,
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
                                                            value: 382,
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
                                                            value: 383,
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
                                                            value: 384,
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
                                                            value: 385,
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
                                                            value: 165,
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
                                                            value: 195,
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
                                                            value: 366,
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
                                                            value: 386,
                                                        },
                                                    ),
                                                ),
                                                17,
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
                                                ident: `simp_one_match`,
                                                access_start: TokenIdx(
                                                    99,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `downmost`,
                                                access_start: TokenIdx(
                                                    157,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `upmost`,
                                                access_start: TokenIdx(
                                                    166,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `hat`,
                                                access_start: TokenIdx(
                                                    175,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `downmost_number_of_strokes`,
                                                access_start: TokenIdx(
                                                    227,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `long_vertical`,
                                                access_start: TokenIdx(
                                                    327,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            402,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `long_vertical_dp`,
                                                access_start: TokenIdx(
                                                    342,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            402,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `downmost_hat`,
                                                access_start: TokenIdx(
                                                    454,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 10,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `downmost_hat_dp`,
                                                access_start: TokenIdx(
                                                    467,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 11,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `downmost_feet`,
                                                access_start: TokenIdx(
                                                    475,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 12,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `downmost_feet_dp`,
                                                access_start: TokenIdx(
                                                    490,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 13,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `a`,
                                                access_start: TokenIdx(
                                                    519,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 14,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `b`,
                                                access_start: TokenIdx(
                                                    525,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 15,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `c`,
                                                access_start: TokenIdx(
                                                    531,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 16,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `d`,
                                                access_start: TokenIdx(
                                                    537,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            553,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 17,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 263,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            263,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                            ast_idx: 70,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    558,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    562,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    559,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    563,
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
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            556,
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        557,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 1,
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
                                        555,
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
                                                557,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        560,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        561,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
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
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            558,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            562,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            559,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            563,
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    556,
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                557,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::Parameter {
                                                                pattern_symbol: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 1,
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
                                        FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                568,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::Parameter,
                                        },
                                        Expr::MethodCall {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                569,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    570,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                571,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                572,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                574,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                575,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    576,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                578,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                577,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                579,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                580,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    581,
                                                ),
                                            },
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        567,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    573,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 7,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        566,
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
                                                            value: 299,
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
                                                ident: `cc`,
                                                kind: InheritedSymbolKind::Parameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `dp`,
                                                access_start: TokenIdx(
                                                    567,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            582,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
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
                        path: FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                            ast_idx: 71,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    587,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    591,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    588,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    592,
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
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            585,
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        586,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 1,
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
                                        584,
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
                                                586,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        589,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        590,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        593,
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
                                                    FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            587,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            591,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            588,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            592,
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    585,
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                586,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::Parameter {
                                                                pattern_symbol: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 1,
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
                                        FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                597,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::Parameter,
                                        },
                                        Expr::MethodCall {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                598,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    599,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                600,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                601,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                603,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                604,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    605,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                607,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                606,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                609,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::Parameter,
                                        },
                                        Expr::MethodCall {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                610,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    611,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                612,
                                            ),
                                            arguments: ArenaIdxRange(
                                                7..7,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                613,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 7,
                                            dot_token_idx: TokenIdx(
                                                614,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    615,
                                                ),
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                608,
                                            ),
                                            opd: 8,
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
                                                    594,
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
                                                        596,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    602,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 9,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        595,
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
                                                            value: 299,
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
                                                ident: `cc`,
                                                kind: InheritedSymbolKind::Parameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `dp`,
                                                access_start: TokenIdx(
                                                    596,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            616,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 10,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            10,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                            ast_idx: 72,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    621,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    625,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    622,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    626,
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
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            619,
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        620,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 1,
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
                                        618,
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
                                                620,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        623,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        624,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        627,
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
                                                    FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            621,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            625,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            622,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            626,
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    619,
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                620,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::Parameter {
                                                                pattern_symbol: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 1,
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
                                        FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                631,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::Parameter,
                                        },
                                        Expr::MethodCall {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                632,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    633,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                634,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                635,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                637,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                638,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    639,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                641,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                640,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                643,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                644,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    645,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                647,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 7,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                646,
                                            ),
                                            ropd: 8,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                649,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::Field {
                                            this_expr: 10,
                                            dot_token_idx: TokenIdx(
                                                650,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    651,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                653,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                648,
                                            ),
                                            opd: 11,
                                        },
                                        Expr::Field {
                                            this_expr: 12,
                                            dot_token_idx: TokenIdx(
                                                654,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    655,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 13,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                652,
                                            ),
                                            ropd: 14,
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
                                                    628,
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
                                                        630,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    636,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    642,
                                                ),
                                            },
                                            condition: Ok(
                                                9,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr: 15,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        629,
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
                                                            value: 299,
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
                                                ident: `cc`,
                                                kind: InheritedSymbolKind::Parameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `dp`,
                                                access_start: TokenIdx(
                                                    630,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            656,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 16,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            16,
                        ),
                    },
                ),
            ),
        ],
    },
)