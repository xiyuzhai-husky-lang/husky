Ok(
    DefnSheet {
        defns: [
            Defn::Form(
                FormDefn::Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                            ast_idx: 57,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        66,
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
                                        68,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
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
                                                            67,
                                                        ),
                                                        ident: `FermiMatchResult`,
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
                                        EntityPath::ModuleItem(
                                            ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
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
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                73,
                                            ),
                                            items: ArenaIdxRange(
                                                1..2,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                75,
                                            ),
                                        },
                                        Expr::RitchieCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                70,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..4,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                76,
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
                                                69,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                71,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                74,
                                            ),
                                            ident: `upmost`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                                                ),
                                            ),
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
            Defn::Form(
                FormDefn::Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                            ast_idx: 58,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        79,
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
                                        81,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
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
                                                            80,
                                                        ),
                                                        ident: `FermiMatchResult`,
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
                                        EntityPath::ModuleItem(
                                            ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
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
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                86,
                                            ),
                                            items: ArenaIdxRange(
                                                1..3,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                90,
                                            ),
                                        },
                                        Expr::RitchieCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                83,
                                            ),
                                            arguments: ArenaIdxRange(
                                                3..5,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                91,
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
                                                82,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                84,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                87,
                                            ),
                                            ident: `upmost`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                89,
                                            ),
                                            ident: `bottom1`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
                                                ),
                                            ),
                                        },
                                    ],
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
            Defn::Form(
                FormDefn::Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                            ast_idx: 59,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        94,
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
                                        97,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
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
                                                            96,
                                                        ),
                                                        ident: `MnistLabel`,
                                                    },
                                                ),
                                            ),
                                            Expr::PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    95,
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
                                        EntityPath::ModuleItem(
                                            ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
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
                                                        99,
                                                    ),
                                                    ident: `is_one`,
                                                },
                                            ),
                                        ),
                                        Expr::Be {
                                            src: 0,
                                            be_token_idx: TokenIdx(
                                                100,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 0,
                                                },
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                106,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    107,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                109,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                3,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                108,
                                            ),
                                            items: ArenaIdxRange(
                                                4..5,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                110,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match`,
                                            token_idx: TokenIdx(
                                                112,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Be {
                                            src: 6,
                                            be_token_idx: TokenIdx(
                                                113,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 2,
                                                },
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                125,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    126,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                129,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 12,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                127,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                132,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 15,
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
                                                16,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                135,
                                            ),
                                            items: ArenaIdxRange(
                                                17..18,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                137,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 18,
                                            be_token_idx: TokenIdx(
                                                138,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 5,
                                                },
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                    ident: `narrow_down`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                    ident: `MnistLabel`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                    ident: `Six`,
                                                },
                                            ),
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 5,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 21,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                144,
                                            ),
                                            ropd: 22,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                147,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 23,
                                            dot_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    151,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 24,
                                            dot_token_idx: TokenIdx(
                                                154,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    155,
                                                ),
                                            },
                                        },
                                        Expr::RitchieCall {
                                            function: 20,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                142,
                                            ),
                                            arguments: ArenaIdxRange(
                                                25..29,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                156,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 29,
                                            opr: Unveil,
                                            opr_token_idx: TokenIdx(
                                                157,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 6,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 31,
                                            dot_token_idx: TokenIdx(
                                                162,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                165,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                32,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                164,
                                            ),
                                            items: ArenaIdxRange(
                                                33..34,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                166,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `bottom1_match`,
                                            token_idx: TokenIdx(
                                                170,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 35,
                                            dot_token_idx: TokenIdx(
                                                171,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    172,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                173,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                36..36,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                174,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `bottom1_match_dp`,
                                            token_idx: TokenIdx(
                                                178,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 37,
                                            dot_token_idx: TokenIdx(
                                                179,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    180,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match`,
                                            token_idx: TokenIdx(
                                                184,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 39,
                                            dot_token_idx: TokenIdx(
                                                185,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    186,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                187,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                40..40,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                188,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 40,
                                            dot_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    190,
                                                ),
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 7,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 42,
                                            dot_token_idx: TokenIdx(
                                                195,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 8,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 44,
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
                                            lopd: 45,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ropd: 46,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `bottom1_match`,
                                            token_idx: TokenIdx(
                                                204,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::Be {
                                            src: 48,
                                            be_token_idx: TokenIdx(
                                                205,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 11,
                                                },
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                212,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `bottom1_match_dp_y`,
                                            token_idx: TokenIdx(
                                                209,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                211,
                                            ),
                                            opd: 50,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 51,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                210,
                                            ),
                                            ropd: 52,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 9,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnterminatedList {
                                                    bra_token_idx: TokenIdx(
                                                        215,
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    ident: `narrow_down`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                    ident: `MnistLabel`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                    ident: `Six`,
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 57,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                220,
                                            ),
                                            ropd: 58,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                223,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match_dp_y`,
                                            token_idx: TokenIdx(
                                                225,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::RitchieCall {
                                            function: 56,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                218,
                                            ),
                                            arguments: ArenaIdxRange(
                                                59..62,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                226,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 62,
                                            opr: Unveil,
                                            opr_token_idx: TokenIdx(
                                                227,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                    ident: `MnistLabel`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                    ident: `Six`,
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 64,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                229,
                                            ),
                                            ropd: 65,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 10,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 67,
                                            dot_token_idx: TokenIdx(
                                                235,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    236,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match`,
                                            token_idx: TokenIdx(
                                                240,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 69,
                                            dot_token_idx: TokenIdx(
                                                241,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    242,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                243,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                70..70,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                244,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            self_argument: 68,
                                            dot_token_idx: TokenIdx(
                                                237,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_point`,
                                                token_idx: TokenIdx(
                                                    238,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                239,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                70..71,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                245,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                    ident: `narrow_down`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        248,
                                                    ),
                                                    ident: `MnistLabel`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                    ident: `Six`,
                                                },
                                            ),
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 12,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                264,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 73,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                249,
                                            ),
                                            ropd: 74,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                252,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match_dp_y`,
                                            token_idx: TokenIdx(
                                                254,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 11,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `lower_excess`,
                                            token_idx: TokenIdx(
                                                258,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 75,
                                            dot_token_idx: TokenIdx(
                                                261,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `top_k_row_span_sum`,
                                                token_idx: TokenIdx(
                                                    262,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                263,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                76..77,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                265,
                                            ),
                                        },
                                        Expr::RitchieCall {
                                            function: 72,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                247,
                                            ),
                                            arguments: ArenaIdxRange(
                                                77..83,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                267,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 83,
                                            opr: Unveil,
                                            opr_token_idx: TokenIdx(
                                                268,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `rel_upmost_match_end`,
                                            token_idx: TokenIdx(
                                                270,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 85,
                                            dot_token_idx: TokenIdx(
                                                271,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    272,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                274,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 86,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                273,
                                            ),
                                            ropd: 87,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        276,
                                                    ),
                                                    ident: `MnistLabel`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        278,
                                                    ),
                                                    ident: `Six`,
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 89,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                277,
                                            ),
                                            ropd: 90,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        279,
                                                    ),
                                                    ident: `MnistLabel`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        281,
                                                    ),
                                                    ident: `Six`,
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 92,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                280,
                                            ),
                                            ropd: 93,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        282,
                                                    ),
                                                    ident: `narrow_down`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        284,
                                                    ),
                                                    ident: `MnistLabel`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        286,
                                                    ),
                                                    ident: `Six`,
                                                },
                                            ),
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 13,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 96,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                285,
                                            ),
                                            ropd: 97,
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                288,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 98,
                                            dot_token_idx: TokenIdx(
                                                291,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    292,
                                                ),
                                            },
                                        },
                                        Expr::RitchieCall {
                                            function: 95,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                283,
                                            ),
                                            arguments: ArenaIdxRange(
                                                99..102,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                293,
                                            ),
                                        },
                                        Expr::SuffixOpn {
                                            opd: 102,
                                            opr: Unveil,
                                            opr_token_idx: TokenIdx(
                                                294,
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 14,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 104,
                                            dot_token_idx: TokenIdx(
                                                297,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    298,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                300,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 105,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                299,
                                            ),
                                            ropd: 106,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 15,
                                            entity_path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 108,
                                            dot_token_idx: TokenIdx(
                                                304,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    305,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                307,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 109,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                306,
                                            ),
                                            ropd: 110,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                309,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 112,
                                            dot_token_idx: TokenIdx(
                                                310,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    311,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                313,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                113,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                312,
                                            ),
                                            items: ArenaIdxRange(
                                                114..115,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                314,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 115,
                                            dot_token_idx: TokenIdx(
                                                315,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    316,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 116,
                                            dot_token_idx: TokenIdx(
                                                317,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    318,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                319,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                117..117,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                320,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                322,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 117,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                321,
                                            ),
                                            ropd: 118,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                324,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 120,
                                            dot_token_idx: TokenIdx(
                                                325,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    326,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                328,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                121,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                327,
                                            ),
                                            items: ArenaIdxRange(
                                                122..123,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                329,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 123,
                                            be_token_idx: TokenIdx(
                                                330,
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
                                                334,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 125,
                                            dot_token_idx: TokenIdx(
                                                335,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
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
                                        Expr::NewBoxList {
                                            caller: Some(
                                                126,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                337,
                                            ),
                                            items: ArenaIdxRange(
                                                127..128,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                339,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 128,
                                            dot_token_idx: TokenIdx(
                                                340,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    341,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 129,
                                            dot_token_idx: TokenIdx(
                                                342,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    343,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                344,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                130..130,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                345,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                347,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 130,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                346,
                                            ),
                                            ropd: 131,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `lower_excess`,
                                            token_idx: TokenIdx(
                                                349,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                351,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 133,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                350,
                                            ),
                                            ropd: 134,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        352,
                                                    ),
                                                    ident: `MnistLabel`,
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                OriginalExprError::UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        354,
                                                    ),
                                                    ident: `Six`,
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 136,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                353,
                                            ),
                                            ropd: 137,
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
                                                105,
                                            ),
                                            ident: `six_match`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                118,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                124,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                128,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                            ident: `six_match`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                153,
                                            ),
                                            ident: `six_match_refined1`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                161,
                                            ),
                                            ident: `six_match_refined1`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                194,
                                            ),
                                            ident: `six_match_refined1`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                198,
                                            ),
                                            ident: `six_match_refined1`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                214,
                                            ),
                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                234,
                                            ),
                                            ident: `major_line_segment_sketch`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                256,
                                            ),
                                            ident: `ignored_connected_components_row_span_sum_sum`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                260,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                290,
                                            ),
                                            ident: `six_match`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                296,
                                            ),
                                            ident: `six_match`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                303,
                                            ),
                                            ident: `six_match`,
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::six_match`, `Feature`),
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
                                                    208,
                                                ),
                                            },
                                            condition: Ok(
                                                53,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    213,
                                                ),
                                            },
                                            condition: Ok(
                                                55,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 63,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 66,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 91,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 30,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    158,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                34,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    167,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        169,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                36,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    175,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                38,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    181,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        183,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                41,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    191,
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
                                                        193,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                43,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    197,
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
                                                        203,
                                                    ),
                                                },
                                                condition: Ok(
                                                    49,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            207,
                                                        ),
                                                    },
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
                                                    231,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        233,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                71,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 84,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                },
                                                condition: Ok(
                                                    88,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            275,
                                                        ),
                                                    },
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
                                            expr_idx: 94,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    302,
                                                ),
                                            },
                                            condition: Ok(
                                                111,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    333,
                                                ),
                                            },
                                            condition: Ok(
                                                132,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    98,
                                                ),
                                            },
                                            condition: Ok(
                                                1,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    102,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        104,
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
                                                    111,
                                                ),
                                            },
                                            condition: Ok(
                                                7,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    115,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        117,
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
                                                    121,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                14,
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                },
                                                condition: Ok(
                                                    19,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            140,
                                                        ),
                                                    },
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
                                            expr_idx: 103,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        295,
                                                    ),
                                                },
                                                condition: Ok(
                                                    107,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            301,
                                                        ),
                                                    },
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
                                                    308,
                                                ),
                                            },
                                            condition: Ok(
                                                119,
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        323,
                                                    ),
                                                },
                                                condition: Ok(
                                                    124,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            332,
                                                        ),
                                                    },
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
                                                    348,
                                                ),
                                            },
                                            condition: Ok(
                                                135,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 138,
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
                                                        101,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        114,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        116,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        122,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `bottom1_match`,
                                                    token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `bottom1_match_dp`,
                                                    token_idx: TokenIdx(
                                                        168,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `bottom1_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        176,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `upmost_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        182,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `rel_upmost_match_end`,
                                                    token_idx: TokenIdx(
                                                        232,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        331,
                                                    ),
                                                },
                                                liason: None,
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
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 407,
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
                                                            value: 436,
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
                                                            value: 410,
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
                                                            value: 185,
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
                                                            value: 437,
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
                                                            value: 407,
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
                                                            value: 439,
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
                                                            value: 440,
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
                                                            value: 441,
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
                                                            value: 442,
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
                                                            value: 392,
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
                                                            value: 410,
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
                                                            value: 443,
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
                                                            value: 410,
                                                        },
                                                    ),
                                                ),
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
                                                ident: `upmost_match`,
                                                access_start: TokenIdx(
                                                    104,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            355,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `eff_holes`,
                                                access_start: TokenIdx(
                                                    117,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            355,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `lower_excess`,
                                                access_start: TokenIdx(
                                                    123,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            355,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `bottom1_match`,
                                                access_start: TokenIdx(
                                                    160,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            282,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `bottom1_match_dp`,
                                                access_start: TokenIdx(
                                                    169,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            282,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `bottom1_match_dp_y`,
                                                access_start: TokenIdx(
                                                    177,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            282,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `upmost_match_dp_y`,
                                                access_start: TokenIdx(
                                                    183,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            282,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `others`,
                                                access_start: TokenIdx(
                                                    193,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            282,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 10,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `rel_upmost_match_end`,
                                                access_start: TokenIdx(
                                                    233,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            282,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
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
                                        expr: 139,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            139,
                        ),
                    },
                ),
            ),
            Defn::Form(
                FormDefn::Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                            ast_idx: 60,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    360,
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
                                                    364,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    361,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    365,
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
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            358,
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
                                                                value: 224,
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
                                                        359,
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
                                        357,
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
                                                359,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        362,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        363,
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
                                        366,
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
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            360,
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
                                                            364,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            361,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            365,
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    358,
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
                                                                        value: 224,
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
                                                                359,
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
                                        EntityPath::ModuleItem(
                                            ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::digits::six::upmost`, `Function`),
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                370,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                371,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    372,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                373,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                374,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                376,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                377,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    378,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                380,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                379,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                381,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                382,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    383,
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
                                                    367,
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
                                                        369,
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
                                                    375,
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
                                                        368,
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
                                                            value: 344,
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
                                                    369,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            384,
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
            Defn::Form(
                FormDefn::Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
                            ast_idx: 61,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    389,
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
                                                    393,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    390,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    394,
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
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            387,
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
                                                                value: 224,
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
                                                        388,
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
                                        386,
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
                                                388,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        391,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        392,
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
                                        395,
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
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            389,
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
                                                            393,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            390,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            394,
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    387,
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
                                                                        value: 224,
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
                                                                388,
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
                                        EntityPath::ModuleItem(
                                            ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::digits::six::bottom1`, `Function`),
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                399,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                400,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    401,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                402,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                403,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                405,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                410,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                406,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    407,
                                                ),
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                409,
                                            ),
                                            opd: 3,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 4,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                408,
                                            ),
                                            ropd: 5,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                414,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                418,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                415,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    416,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                419,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    420,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 9,
                                            opr: PureClosed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                417,
                                            ),
                                            ropd: 10,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                413,
                                            ),
                                            item: 11,
                                            rpar_token_idx: TokenIdx(
                                                421,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            self_argument: 12,
                                            dot_token_idx: TokenIdx(
                                                422,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    423,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                424,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                13..13,
                                            ),
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
                                            lopd: 13,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                426,
                                            ),
                                            ropd: 14,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                429,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 16,
                                            dot_token_idx: TokenIdx(
                                                430,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    431,
                                                ),
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 17,
                                            dot_token_idx: TokenIdx(
                                                432,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    433,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                434,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                18..18,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                435,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                437,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 18,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                436,
                                            ),
                                            ropd: 19,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                441,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                442,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    443,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 22,
                                            dot_token_idx: TokenIdx(
                                                444,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    445,
                                                ),
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                449,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 24,
                                            dot_token_idx: TokenIdx(
                                                450,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    451,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                452,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                25..25,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                453,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            self_argument: 23,
                                            dot_token_idx: TokenIdx(
                                                446,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_point`,
                                                token_idx: TokenIdx(
                                                    447,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                448,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                25..26,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                454,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `relative_end`,
                                            token_idx: TokenIdx(
                                                456,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 27,
                                            dot_token_idx: TokenIdx(
                                                457,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    458,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                460,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 28,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                459,
                                            ),
                                            ropd: 29,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                462,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 31,
                                            dot_token_idx: TokenIdx(
                                                463,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    464,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                465,
                                            ),
                                            nonself_arguments: ArenaIdxRange(
                                                32..32,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                466,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 32,
                                            dot_token_idx: TokenIdx(
                                                467,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    468,
                                                ),
                                            },
                                        },
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                461,
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
                                                    412,
                                                ),
                                            },
                                            condition: Ok(
                                                15,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    396,
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
                                                        398,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        404,
                                                    ),
                                                },
                                                condition: Ok(
                                                    6,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            411,
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    428,
                                                ),
                                            },
                                            condition: Ok(
                                                20,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    438,
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
                                                        440,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                26,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    455,
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
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        397,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `relative_end`,
                                                    token_idx: TokenIdx(
                                                        439,
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
                                                            value: 344,
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
                                                            value: 444,
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
                                                    398,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            469,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `relative_end`,
                                                access_start: TokenIdx(
                                                    440,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            469,
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
        ],
    },
)