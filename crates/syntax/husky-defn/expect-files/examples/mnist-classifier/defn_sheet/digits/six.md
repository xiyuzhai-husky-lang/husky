Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                        ),
                    ),
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    6,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                        ),
                    ),
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    19,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::six::is_six`, `Var`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::six::is_six`, `Var`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::six::is_six`, `Var`),
                                    ast_idx: 52,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                38,
                                            ),
                                        ),
                                    ),
                                    var_ty: Some(
                                        FormTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            41,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    42,
                                                ),
                                            },
                                        ),
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::is_six`, `Var`),
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
                                                                    40,
                                                                ),
                                                                ident: `MnistLabel`,
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            39,
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
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: VarType,
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
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::is_six`, `Var`),
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
                                                                44,
                                                            ),
                                                            ident: `is_one`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Be {
                                                    src: 0,
                                                    be_token_idx: TokenIdx(
                                                        45,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 0,
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        51,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        54,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 3,
                                                    lbox_token_idx: TokenIdx(
                                                        53,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        55,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 6,
                                                    be_token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 2,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                63,
                                                            ),
                                                            ident: `major_connected_component`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 8,
                                                    dot_token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            65,
                                                        ),
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                69,
                                                            ),
                                                            ident: `major_connected_component`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                73,
                                                            ),
                                                            ident: `major_connected_component`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 10,
                                                    dot_token_idx: TokenIdx(
                                                        70,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            71,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 11,
                                                    dot_token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            75,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 12,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                    ropd: 13,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        77,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 15,
                                                    dot_token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            79,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        81,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 16,
                                                    lbox_token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        82,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 18,
                                                    be_token_idx: TokenIdx(
                                                        83,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 5,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                86,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                88,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                90,
                                                            ),
                                                            ident: `Six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 21,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        89,
                                                    ),
                                                    ropd: 22,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        92,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 23,
                                                    dot_token_idx: TokenIdx(
                                                        95,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            96,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 24,
                                                    dot_token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            100,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 20,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        25..29,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            91,
                                                        ),
                                                        TokenIdx(
                                                            93,
                                                        ),
                                                        TokenIdx(
                                                            97,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 29,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        102,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 31,
                                                    dot_token_idx: TokenIdx(
                                                        107,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            108,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        110,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 32,
                                                    lbox_token_idx: TokenIdx(
                                                        109,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        33..34,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        111,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match`,
                                                    token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 35,
                                                    dot_token_idx: TokenIdx(
                                                        116,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        118,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        36..36,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        119,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match_dp`,
                                                    token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 37,
                                                    dot_token_idx: TokenIdx(
                                                        124,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            125,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 39,
                                                    dot_token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            131,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        40..40,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 40,
                                                    dot_token_idx: TokenIdx(
                                                        134,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            135,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 42,
                                                    dot_token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `others`,
                                                        token_idx: TokenIdx(
                                                            141,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 5,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 44,
                                                    dot_token_idx: TokenIdx(
                                                        144,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            145,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        147,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 45,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        146,
                                                    ),
                                                    ropd: 46,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match`,
                                                    token_idx: TokenIdx(
                                                        149,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 48,
                                                    be_token_idx: TokenIdx(
                                                        150,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 11,
                                                        },
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        157,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                    opd: 50,
                                                },
                                                Expr::Binary {
                                                    lopd: 51,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                    ropd: 52,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                159,
                                                            ),
                                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        161,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 54,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                    ropd: 55,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                162,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                164,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                166,
                                                            ),
                                                            ident: `Six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 58,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    ropd: 59,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        168,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 57,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        163,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        60..63,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            167,
                                                        ),
                                                        TokenIdx(
                                                            169,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        171,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 63,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        172,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                173,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                175,
                                                            ),
                                                            ident: `Six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 65,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                    ropd: 66,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                179,
                                                            ),
                                                            ident: `major_line_segment_sketch`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 68,
                                                    dot_token_idx: TokenIdx(
                                                        180,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            181,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 70,
                                                    dot_token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            187,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        71..71,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 69,
                                                    dot_token_idx: TokenIdx(
                                                        182,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_point`,
                                                        token_idx: TokenIdx(
                                                            183,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        184,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        71..72,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        190,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                191,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                193,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                195,
                                                            ),
                                                            ident: `Six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                205,
                                                            ),
                                                            ident: `major_connected_component`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        209,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 74,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                    ropd: 75,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        197,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        199,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                201,
                                                            ),
                                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 76,
                                                    dot_token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            207,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        77..78,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 73,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        78..84,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            196,
                                                        ),
                                                        TokenIdx(
                                                            198,
                                                        ),
                                                        TokenIdx(
                                                            200,
                                                        ),
                                                        TokenIdx(
                                                            202,
                                                        ),
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                        TokenIdx(
                                                            211,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 84,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `rel_upmost_match_end`,
                                                    token_idx: TokenIdx(
                                                        215,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 86,
                                                    dot_token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            217,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        219,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 87,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    ropd: 88,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                221,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                223,
                                                            ),
                                                            ident: `Six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 90,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                    ropd: 91,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                224,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                226,
                                                            ),
                                                            ident: `Six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 93,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        225,
                                                    ),
                                                    ropd: 94,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                227,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                229,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                231,
                                                            ),
                                                            ident: `Six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 6,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 97,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                    ropd: 98,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        233,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 99,
                                                    dot_token_idx: TokenIdx(
                                                        236,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            237,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 96,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        100..103,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            232,
                                                        ),
                                                        TokenIdx(
                                                            234,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        238,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 103,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        239,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 7,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 105,
                                                    dot_token_idx: TokenIdx(
                                                        242,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            243,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        245,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 106,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        244,
                                                    ),
                                                    ropd: 107,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 8,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 109,
                                                    dot_token_idx: TokenIdx(
                                                        249,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            250,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        252,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 110,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        251,
                                                    ),
                                                    ropd: 111,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        254,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 113,
                                                    dot_token_idx: TokenIdx(
                                                        255,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            256,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        258,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 114,
                                                    lbox_token_idx: TokenIdx(
                                                        257,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        115..116,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        259,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 116,
                                                    dot_token_idx: TokenIdx(
                                                        260,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            261,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 117,
                                                    dot_token_idx: TokenIdx(
                                                        262,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            263,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        264,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        118..118,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        265,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        267,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 118,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        266,
                                                    ),
                                                    ropd: 119,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 121,
                                                    dot_token_idx: TokenIdx(
                                                        270,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            271,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        273,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 122,
                                                    lbox_token_idx: TokenIdx(
                                                        272,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        123..124,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        274,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 124,
                                                    be_token_idx: TokenIdx(
                                                        275,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 13,
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        279,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 126,
                                                    dot_token_idx: TokenIdx(
                                                        280,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            281,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        283,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 127,
                                                    lbox_token_idx: TokenIdx(
                                                        282,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        128..129,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        284,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 129,
                                                    dot_token_idx: TokenIdx(
                                                        285,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            286,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 130,
                                                    dot_token_idx: TokenIdx(
                                                        287,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            288,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        289,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        131..131,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        290,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        292,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 131,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        291,
                                                    ),
                                                    ropd: 132,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        294,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        296,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 134,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        295,
                                                    ),
                                                    ropd: 135,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                297,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                299,
                                                            ),
                                                            ident: `Six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 137,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        298,
                                                    ),
                                                    ropd: 138,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        19..31,
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
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        235,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        241,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        248,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            153,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        53,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            158,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        56,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 64,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 67,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 92,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 30,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            103,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 6,
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
                                                                105,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        34,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 7,
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
                                                                114,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        36,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            120,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 8,
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
                                                                122,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        38,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            126,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 9,
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
                                                                128,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        41,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            136,
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
                                                        EqToken(
                                                            TokenIdx(
                                                                138,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        43,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            142,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        47,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                148,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            49,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        152,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                0..4,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            176,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 12,
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
                                                                178,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        72,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 85,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                214,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            89,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        220,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 95,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            247,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        112,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            278,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        133,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            43,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 1,
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
                                                                49,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        7,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            60,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 3,
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
                                                                62,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        9,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            66,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 4,
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
                                                        EqToken(
                                                            TokenIdx(
                                                                68,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        14,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                76,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            19,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        85,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                5..17,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 104,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                240,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            108,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        246,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                17..18,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            253,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        120,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                268,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            125,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        277,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                18..19,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            293,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        136,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 139,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                46,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `upmost_match`,
                                                            token_idx: TokenIdx(
                                                                48,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                59,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `eff_holes`,
                                                            token_idx: TokenIdx(
                                                                61,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `lower_excess`,
                                                            token_idx: TokenIdx(
                                                                67,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                84,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match`,
                                                            token_idx: TokenIdx(
                                                                104,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match_dp`,
                                                            token_idx: TokenIdx(
                                                                113,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match_dp_y`,
                                                            token_idx: TokenIdx(
                                                                121,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `upmost_match_dp_y`,
                                                            token_idx: TokenIdx(
                                                                127,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                137,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                151,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `rel_upmost_match_end`,
                                                            token_idx: TokenIdx(
                                                                177,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                276,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
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
                                                Be,
                                                Let,
                                                Be,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `none`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `upmost_match`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `eff_holes`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `lower_excess`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `none`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `bottom1_match`,
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `bottom1_match_dp`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `bottom1_match_dp_y`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `upmost_match_dp_y`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `others`,
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        11,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `rel_upmost_match_end`,
                                                        12,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        13,
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
                                                            49,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    300,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `upmost_match`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            62,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    300,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `eff_holes`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            68,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    300,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `lower_excess`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            105,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    227,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `bottom1_match`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            114,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    227,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `bottom1_match_dp`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            122,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    227,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `bottom1_match_dp_y`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            128,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    227,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `upmost_match_dp_y`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            138,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    227,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `others`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            178,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    227,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `rel_upmost_match_end`,
                                                            pattern_symbol_idx: 12,
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
                                                expr: 140,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    140,
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
                            FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                    ast_idx: 53,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                                                    306,
                                                                ),
                                                                ident: `ConcaveComponent`,
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            305,
                                                        ),
                                                        opd: 0,
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
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            309,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            310,
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    303,
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
                                                            `cc`,
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
                                                                304,
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
                                                302,
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
                                                        304,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                307,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                308,
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
                                                311,
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
                                                                    FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                                                            306,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                    },
                                                                ),
                                                            ),
                                                            Expr::Prefix {
                                                                opr: Tilde,
                                                                opr_token_idx: TokenIdx(
                                                                    305,
                                                                ),
                                                                opd: 0,
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
                                                            Expr::Prefix {
                                                                opr: Option,
                                                                opr_token_idx: TokenIdx(
                                                                    309,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    310,
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
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            303,
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
                                                                    `cc`,
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
                                                                        304,
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
                                                        FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        315,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        316,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            317,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        318,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        319,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        321,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        322,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            323,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        325,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        324,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        326,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        327,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            328,
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
                                                            312,
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
                                                                314,
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
                                                            320,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
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
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                313,
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
                                                        `dp`,
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
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            314,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    329,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `dp`,
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
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                    ast_idx: 54,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                                                                    335,
                                                                ),
                                                                ident: `ConcaveComponent`,
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            334,
                                                        ),
                                                        opd: 0,
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
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            338,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            339,
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    332,
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
                                                            `cc`,
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
                                                                333,
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
                                                331,
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
                                                        333,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                336,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                337,
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
                                                340,
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
                                                                    FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                                                                            335,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                    },
                                                                ),
                                                            ),
                                                            Expr::Prefix {
                                                                opr: Tilde,
                                                                opr_token_idx: TokenIdx(
                                                                    334,
                                                                ),
                                                                opd: 0,
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
                                                            Expr::Prefix {
                                                                opr: Option,
                                                                opr_token_idx: TokenIdx(
                                                                    338,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    339,
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
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            332,
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
                                                                    `cc`,
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
                                                                        333,
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
                                                        FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        344,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        345,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            346,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        347,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        348,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        350,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        355,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        351,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            352,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        354,
                                                    ),
                                                    opd: 3,
                                                },
                                                Expr::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        353,
                                                    ),
                                                    ropd: 5,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        359,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        363,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 7,
                                                    dot_token_idx: TokenIdx(
                                                        360,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            361,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 8,
                                                    dot_token_idx: TokenIdx(
                                                        364,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            365,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 9,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        362,
                                                    ),
                                                    ropd: 10,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        358,
                                                    ),
                                                    item: 11,
                                                    rpar_token_idx: TokenIdx(
                                                        366,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 12,
                                                    dot_token_idx: TokenIdx(
                                                        367,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            368,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        369,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        13..13,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        370,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        372,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 13,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        371,
                                                    ),
                                                    ropd: 14,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        374,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 16,
                                                    dot_token_idx: TokenIdx(
                                                        375,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            376,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 17,
                                                    dot_token_idx: TokenIdx(
                                                        377,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            378,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        379,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        18..18,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        380,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        382,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 18,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        381,
                                                    ),
                                                    ropd: 19,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        386,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 21,
                                                    dot_token_idx: TokenIdx(
                                                        387,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `line_segment_sketch`,
                                                        token_idx: TokenIdx(
                                                            388,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 22,
                                                    dot_token_idx: TokenIdx(
                                                        389,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            390,
                                                        ),
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        394,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 24,
                                                    dot_token_idx: TokenIdx(
                                                        395,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            396,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        397,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        25..25,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        398,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 23,
                                                    dot_token_idx: TokenIdx(
                                                        391,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_point`,
                                                        token_idx: TokenIdx(
                                                            392,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        393,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        25..26,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        399,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `relative_end`,
                                                    token_idx: TokenIdx(
                                                        401,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 27,
                                                    dot_token_idx: TokenIdx(
                                                        402,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            403,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        405,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 28,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        404,
                                                    ),
                                                    ropd: 29,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        407,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 31,
                                                    dot_token_idx: TokenIdx(
                                                        408,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            409,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        410,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        32..32,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        411,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 32,
                                                    dot_token_idx: TokenIdx(
                                                        412,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            413,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        406,
                                                    ),
                                                    opd: 33,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..7,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            357,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        15,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            341,
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
                                                                343,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                349,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            6,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        356,
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
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            373,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        20,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            383,
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
                                                                385,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        26,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            400,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        30,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 34,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                342,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `relative_end`,
                                                            token_idx: TokenIdx(
                                                                384,
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
                                                        `dp`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `relative_end`,
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
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            343,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    414,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `dp`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            385,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    414,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `relative_end`,
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
                                                expr: 35,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    35,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)