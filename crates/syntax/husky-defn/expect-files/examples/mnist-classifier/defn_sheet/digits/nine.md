Ok(
    DefnSheet {
        defns: [
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                            ast_idx: 44,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        76,
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
                                        78,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            77,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 362,
                                                                },
                                                            ),
                                                        ),
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
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
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
                                                FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
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
                                                83,
                                            ),
                                            items: ArenaIdxRange(
                                                1..2,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                85,
                                            ),
                                        },
                                        Expr::RitchieCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                80,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..4,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                86,
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
                                                79,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                81,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                84,
                                            ),
                                            ident: `downmost`,
                                            entity_path: FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
                                        },
                                    ],
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
                                    allow_self_type: False,
                                    allow_self_value: False,
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
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                            ast_idx: 45,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        89,
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
                                        91,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 362,
                                                                },
                                                            ),
                                                        ),
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
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
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
                                                FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
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
                                                96,
                                            ),
                                            items: ArenaIdxRange(
                                                1..2,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                98,
                                            ),
                                        },
                                        Expr::RitchieCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                93,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..4,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                99,
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
                                                92,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                94,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                97,
                                            ),
                                            ident: `big_cc`,
                                            entity_path: FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
                                        },
                                    ],
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
                                    allow_self_type: False,
                                    allow_self_value: False,
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
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                            ast_idx: 46,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        102,
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
                                        105,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            104,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 123,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                            Expr::PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    103,
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
                                            kind: ReturnType,
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
                                        FormPath(`mnist_classifier::digits::nine::is_nine`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        107,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 132,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Be {
                                            src: 0,
                                            be_token_idx: TokenIdx(
                                                108,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 0,
                                                },
                                            ),
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        111,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 131,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Be {
                                            src: 2,
                                            be_token_idx: TokenIdx(
                                                112,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 1,
                                                },
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                118,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    119,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                121,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                122,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    123,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                125,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                7,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                124,
                                            ),
                                            items: ArenaIdxRange(
                                                8..9,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                126,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 9,
                                            be_token_idx: TokenIdx(
                                                127,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 3,
                                                },
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                133,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                136,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                12,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                135,
                                            ),
                                            items: ArenaIdxRange(
                                                13..14,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                137,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `down_match`,
                                            token_idx: TokenIdx(
                                                139,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::Be {
                                            src: 15,
                                            be_token_idx: TokenIdx(
                                                140,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 5,
                                                },
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `down_match`,
                                            token_idx: TokenIdx(
                                                145,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 17,
                                            dot_token_idx: TokenIdx(
                                                146,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    147,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                148,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                18..18,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                149,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 18,
                                            dot_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    151,
                                                ),
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 20,
                                            dot_token_idx: TokenIdx(
                                                156,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    157,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                160,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    161,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 22,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                158,
                                            ),
                                            ropd: 23,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `higher_excess`,
                                            token_idx: TokenIdx(
                                                163,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                165,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 25,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                164,
                                            ),
                                            ropd: 26,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                167,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 28,
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
                                                29,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                170,
                                            ),
                                            items: ArenaIdxRange(
                                                30..31,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                172,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 31,
                                            be_token_idx: TokenIdx(
                                                173,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 8,
                                                },
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        Expr::MethodCall {
                                            self_argument: 33,
                                            dot_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    179,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                180,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                34..34,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                181,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                183,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 34,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ropd: 35,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 5,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 37,
                                            dot_token_idx: TokenIdx(
                                                188,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    189,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                191,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                38,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                190,
                                            ),
                                            items: ArenaIdxRange(
                                                39..40,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                192,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `nine_match_refine_result`,
                                            token_idx: TokenIdx(
                                                194,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::Be {
                                            src: 41,
                                            be_token_idx: TokenIdx(
                                                195,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 10,
                                                },
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 6,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 43,
                                            dot_token_idx: TokenIdx(
                                                199,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    200,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                202,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 44,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ropd: 45,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 7,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 8,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 47,
                                            dot_token_idx: TokenIdx(
                                                207,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    208,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 48,
                                            dot_token_idx: TokenIdx(
                                                211,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    212,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 49,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                209,
                                            ),
                                            ropd: 50,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 9,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 52,
                                            dot_token_idx: TokenIdx(
                                                217,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    218,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                220,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                53,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                219,
                                            ),
                                            items: ArenaIdxRange(
                                                54..55,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                221,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                223,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::Be {
                                            src: 56,
                                            be_token_idx: TokenIdx(
                                                224,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 13,
                                                },
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                227,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 58,
                                            dot_token_idx: TokenIdx(
                                                228,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                230,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                59..59,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                231,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 59,
                                            dot_token_idx: TokenIdx(
                                                232,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    233,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                235,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 60,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                234,
                                            ),
                                            ropd: 61,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                237,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                242,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 63,
                                            dot_token_idx: TokenIdx(
                                                238,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    239,
                                                ),
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                241,
                                            ),
                                            opd: 64,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 65,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                240,
                                            ),
                                            ropd: 66,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 10,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 68,
                                            dot_token_idx: TokenIdx(
                                                245,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    246,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                248,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 69,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                247,
                                            ),
                                            ropd: 70,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 11,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                256,
                                            ),
                                        ),
                                        Expr::MethodCall {
                                            self_argument: 72,
                                            dot_token_idx: TokenIdx(
                                                253,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `top_k_row_right_mass_sum`,
                                                token_idx: TokenIdx(
                                                    254,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                255,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                73..74,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                257,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                259,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                261,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 75,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                260,
                                            ),
                                            ropd: 76,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                263,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                265,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 78,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                264,
                                            ),
                                            ropd: 79,
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        266,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 123,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            Original(
                                                UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        268,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 452,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 81,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                267,
                                            ),
                                            ropd: 82,
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 123,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            Original(
                                                UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        271,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 452,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 84,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                270,
                                            ),
                                            ropd: 85,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                14..25,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                117,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                132,
                                            ),
                                            ident: `nine_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::nine::nine_match`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                155,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                159,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                177,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                187,
                                            ),
                                            ident: `nine_match_refine`,
                                            entity_path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                198,
                                            ),
                                            ident: `nine_match_refine`,
                                            entity_path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                206,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                210,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                216,
                                            ),
                                            ident: `nine_match_refine`,
                                            entity_path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                244,
                                            ),
                                            ident: `nine_match_refine`,
                                            entity_path: FormPath(`mnist_classifier::digits::nine::nine_match_refine`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                252,
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
                                                    176,
                                                ),
                                            },
                                            condition: Ok(
                                                36,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    184,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 9,
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
                                                        186,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                40,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    193,
                                                ),
                                            },
                                            condition: Ok(
                                                42,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    197,
                                                ),
                                            },
                                            condition: Ok(
                                                46,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    203,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 11,
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
                                                        205,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                51,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    213,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 12,
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
                                                        215,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                55,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    222,
                                                ),
                                            },
                                            condition: Ok(
                                                57,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    226,
                                                ),
                                            },
                                            condition: Ok(
                                                62,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    236,
                                                ),
                                            },
                                            condition: Ok(
                                                67,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    243,
                                                ),
                                            },
                                            condition: Ok(
                                                71,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    249,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 14,
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
                                                        251,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                74,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    258,
                                                ),
                                            },
                                            condition: Ok(
                                                77,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    262,
                                                ),
                                            },
                                            condition: Ok(
                                                80,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 83,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    106,
                                                ),
                                            },
                                            condition: Ok(
                                                1,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    110,
                                                ),
                                            },
                                            condition: Ok(
                                                3,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 2,
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
                                                        116,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                5,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                            },
                                            condition: Ok(
                                                10,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    129,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 4,
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
                                                        131,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                14,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    138,
                                                ),
                                            },
                                            condition: Ok(
                                                16,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 6,
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
                                                        144,
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
                                                    152,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 7,
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
                                                        154,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                24,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    162,
                                                ),
                                            },
                                            condition: Ok(
                                                27,
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                },
                                                condition: Ok(
                                                    32,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            175,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..14,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 86,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        109,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        113,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `down_match`,
                                                    token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `down_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `higher_excess`,
                                                    token_idx: TokenIdx(
                                                        153,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `nine_match_refine_result`,
                                                    token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `higher_excess`,
                                                    token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `upper_arc`,
                                                    token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        225,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Be,
                                        Be,
                                        Let,
                                        Be,
                                        Let,
                                        Be,
                                        Let,
                                        Let,
                                        Be,
                                        Let,
                                        Be,
                                        Let,
                                        Let,
                                        Be,
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 377,
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
                                                            value: 377,
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
                                                            value: 156,
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
                                                            value: 377,
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
                                                            value: 431,
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
                                                            value: 380,
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
                                                            value: 432,
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
                                                            value: 433,
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
                                                            value: 377,
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
                                                            value: 451,
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
                                                            value: 380,
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
                                                            value: 433,
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
                                                            value: 435,
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
                                                            value: 380,
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
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                                14,
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
                                                ident: `eff_holes`,
                                                access_start: TokenIdx(
                                                    116,
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
                                                ident: `down_match`,
                                                access_start: TokenIdx(
                                                    131,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            272,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `down_match_dp_y`,
                                                access_start: TokenIdx(
                                                    144,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            272,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `higher_excess`,
                                                access_start: TokenIdx(
                                                    154,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            272,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `nine_match_refine_result`,
                                                access_start: TokenIdx(
                                                    186,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            269,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `higher_excess`,
                                                access_start: TokenIdx(
                                                    205,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            269,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 11,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `upper_arc`,
                                                access_start: TokenIdx(
                                                    215,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            269,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 12,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `a`,
                                                access_start: TokenIdx(
                                                    251,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            269,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 14,
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
                                        expr: 87,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            87,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
                            ast_idx: 47,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
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
                                                    277,
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
                                                    281,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    278,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    282,
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
                                                            275,
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
                                                                value: 195,
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
                                                        276,
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
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        274,
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
                                                276,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        279,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        280,
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
                                        283,
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
                                                    FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
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
                                                            277,
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
                                                            281,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            278,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            282,
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
                                                                    275,
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
                                                                        value: 195,
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
                                                                276,
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
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::nine::downmost`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                287,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                288,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    289,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                290,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                291,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                293,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                294,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    295,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                297,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                296,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                298,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                299,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    300,
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
                                                    284,
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
                                                        286,
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
                                                    292,
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
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        285,
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
                                                            value: 315,
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
                                                ident: `dp`,
                                                access_start: TokenIdx(
                                                    286,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            301,
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
                        path: FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
                            ast_idx: 48,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
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
                                                    306,
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
                                                    310,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    307,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    311,
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
                                                            304,
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
                                                                value: 195,
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
                                                        305,
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
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        303,
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
                                                305,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        308,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        309,
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
                                        312,
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
                                                    FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
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
                                                            306,
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
                                                            310,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            307,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            311,
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
                                                                    304,
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
                                                                        value: 195,
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
                                                                305,
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
                                ),
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::nine::big_cc`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                316,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                317,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    318,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                319,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                320,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                322,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                323,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    324,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                326,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                325,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                328,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                329,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    330,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 7,
                                            dot_token_idx: TokenIdx(
                                                331,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    332,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                333,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                8..8,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                334,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                336,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 8,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                335,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                337,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                338,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    339,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 12,
                                            dot_token_idx: TokenIdx(
                                                340,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    341,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                342,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                13..13,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                343,
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
                                                    313,
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
                                                        315,
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
                                                    321,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    327,
                                                ),
                                            },
                                            condition: Ok(
                                                10,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 13,
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
                                                        314,
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
                                                            value: 315,
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
                                                ident: `dp`,
                                                access_start: TokenIdx(
                                                    315,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            344,
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
                                        expr: 14,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            14,
                        ),
                    },
                ),
            ),
        ],
    },
)