Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                    ast_idx: 50,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                6,
                                            ),
                                        ),
                                    ),
                                    var_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            8,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    9,
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
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            7,
                                                        ),
                                                        ident: `FermiMatchResult`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                allow_self_type: False,
                                                allow_self_value: False,
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
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..4,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            13,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        17,
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
                                                        10,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        12,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        15,
                                                    ),
                                                    ident: `upmost`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                body: Some(
                                    5,
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
                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    ast_idx: 51,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                20,
                                            ),
                                        ),
                                    ),
                                    var_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            22,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    23,
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
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            21,
                                                        ),
                                                        ident: `FermiMatchResult`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                allow_self_type: False,
                                                allow_self_value: False,
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
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        28,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..3,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        32,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        25,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..5,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            27,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        33,
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
                                                        24,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        29,
                                                    ),
                                                    ident: `upmost`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        31,
                                                    ),
                                                    ident: `bottom1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                                body: Some(
                                    6,
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
                            FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                    ast_idx: 52,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                40,
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
                                            43,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    44,
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
                                                            FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            41,
                                                        ),
                                                        opd: 0,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            42,
                                                        ),
                                                        ident: `MnistLabel`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
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
                                                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 0,
                                                    be_token_idx: TokenIdx(
                                                        47,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesPattern {
                                                            pattern_expr: 0,
                                                            variables: ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        53,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            54,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        56,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 3,
                                                    lbox_token_idx: TokenIdx(
                                                        55,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        59,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 6,
                                                    be_token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesPattern {
                                                            pattern_expr: 2,
                                                            variables: ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 8,
                                                    dot_token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            67,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 10,
                                                    dot_token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            73,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 11,
                                                    dot_token_idx: TokenIdx(
                                                        76,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            77,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 12,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                    ropd: 13,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 15,
                                                    dot_token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            81,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        83,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 16,
                                                    lbox_token_idx: TokenIdx(
                                                        82,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        84,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 18,
                                                    be_token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesPattern {
                                                            pattern_expr: 5,
                                                            variables: ArenaIdxRange(
                                                                5..6,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                88,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 7,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 8,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 6,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        94,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 21,
                                                    dot_token_idx: TokenIdx(
                                                        97,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            98,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 22,
                                                    dot_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            102,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 20,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        89,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        23..27,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            93,
                                                        ),
                                                        TokenIdx(
                                                            95,
                                                        ),
                                                        TokenIdx(
                                                            99,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 27,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        104,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 9,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 29,
                                                    dot_token_idx: TokenIdx(
                                                        109,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
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
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 30,
                                                    lbox_token_idx: TokenIdx(
                                                        111,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        31..32,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        113,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match`,
                                                    token_idx: TokenIdx(
                                                        117,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 33,
                                                    dot_token_idx: TokenIdx(
                                                        118,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        120,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        34..34,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        121,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match_dp`,
                                                    token_idx: TokenIdx(
                                                        125,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 35,
                                                    dot_token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            127,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 37,
                                                    dot_token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            133,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        134,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        38..38,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        135,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 38,
                                                    dot_token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            137,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 10,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 40,
                                                    dot_token_idx: TokenIdx(
                                                        142,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `others`,
                                                        token_idx: TokenIdx(
                                                            143,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 11,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 42,
                                                    dot_token_idx: TokenIdx(
                                                        146,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            147,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        149,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 43,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                    ropd: 44,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match`,
                                                    token_idx: TokenIdx(
                                                        151,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 46,
                                                    be_token_idx: TokenIdx(
                                                        152,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesPattern {
                                                            pattern_expr: 11,
                                                            variables: ArenaIdxRange(
                                                                11..12,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        159,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        158,
                                                    ),
                                                    opd: 48,
                                                },
                                                Expr::Binary {
                                                    lopd: 49,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                    ropd: 50,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 12,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnterminatedList {
                                                            bra_token_idx: TokenIdx(
                                                                162,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                164,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 14,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        170,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        172,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 54,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        55..58,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            169,
                                                        ),
                                                        TokenIdx(
                                                            171,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 58,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 16,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 17,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 61,
                                                    dot_token_idx: TokenIdx(
                                                        182,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            183,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        187,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 63,
                                                    dot_token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            189,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        190,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        64..64,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 62,
                                                    dot_token_idx: TokenIdx(
                                                        184,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_point`,
                                                        token_idx: TokenIdx(
                                                            185,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        64..65,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                193,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 21,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        211,
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 19,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        199,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 20,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 67,
                                                    dot_token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            209,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        68..69,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 66,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        69..75,
                                                    ),
                                                    commas: [
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
                                                            206,
                                                        ),
                                                        TokenIdx(
                                                            213,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 75,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        215,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `rel_upmost_match_end`,
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 77,
                                                    dot_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            219,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        221,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 78,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                    ropd: 79,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 23,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 25,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                229,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 28,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 27,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        235,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 84,
                                                    dot_token_idx: TokenIdx(
                                                        238,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            239,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 83,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        85..88,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            234,
                                                        ),
                                                        TokenIdx(
                                                            236,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        240,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 88,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        241,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 29,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 90,
                                                    dot_token_idx: TokenIdx(
                                                        244,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            245,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        247,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 91,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                    ropd: 92,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 30,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 94,
                                                    dot_token_idx: TokenIdx(
                                                        251,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            252,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        254,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 95,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        253,
                                                    ),
                                                    ropd: 96,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        256,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 98,
                                                    dot_token_idx: TokenIdx(
                                                        257,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            258,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        260,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 99,
                                                    lbox_token_idx: TokenIdx(
                                                        259,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        100..101,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        261,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 101,
                                                    dot_token_idx: TokenIdx(
                                                        262,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            263,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 102,
                                                    dot_token_idx: TokenIdx(
                                                        264,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            265,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        266,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        103..103,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        267,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        269,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 103,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        268,
                                                    ),
                                                    ropd: 104,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        271,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 106,
                                                    dot_token_idx: TokenIdx(
                                                        272,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            273,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        275,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 107,
                                                    lbox_token_idx: TokenIdx(
                                                        274,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        108..109,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        276,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 109,
                                                    be_token_idx: TokenIdx(
                                                        277,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesPattern {
                                                            pattern_expr: 13,
                                                            variables: ArenaIdxRange(
                                                                13..14,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        281,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 111,
                                                    dot_token_idx: TokenIdx(
                                                        282,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            283,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        285,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 112,
                                                    lbox_token_idx: TokenIdx(
                                                        284,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        113..114,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        286,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 114,
                                                    dot_token_idx: TokenIdx(
                                                        287,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            288,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 115,
                                                    dot_token_idx: TokenIdx(
                                                        289,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            290,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        291,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        116..116,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        292,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        294,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 116,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        293,
                                                    ),
                                                    ropd: 117,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        296,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        298,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 119,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        297,
                                                    ),
                                                    ropd: 120,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 32,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
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
                                                        46,
                                                    ),
                                                    ident: `is_one`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        52,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        65,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        75,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        90,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 5,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            91,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                92,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        96,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 13,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            167,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                168,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 15,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            176,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                177,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                    ident: `major_line_segment_sketch`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        195,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 18,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            196,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                197,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        223,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 22,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                225,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        226,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 24,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            227,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                228,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 26,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            232,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                233,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        237,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        243,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        299,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 31,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            300,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                301,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Six`,
                                                            },
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
                                                            155,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        51,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            160,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        53,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 59,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 60,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 81,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 28,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            105,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 6,
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
                                                                107,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        32,
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
                                                            pattern_expr_idx: 7,
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
                                                                116,
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
                                                            122,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 8,
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
                                                                124,
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
                                                            128,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 9,
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
                                                                130,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        39,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            138,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 10,
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
                                                                140,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        41,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            144,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        45,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                150,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            47,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        154,
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
                                                            178,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 12,
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
                                                        EqToken(
                                                            TokenIdx(
                                                                180,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        65,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 76,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                216,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            80,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        222,
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
                                                    expr_idx: 82,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            249,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        97,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            280,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        118,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 1,
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
                                                                51,
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
                                                            58,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        7,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            62,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 3,
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
                                                                64,
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
                                                            68,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 4,
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
                                                                70,
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
                                                                78,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            19,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        87,
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
                                                    expr_idx: 89,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                242,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            93,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        248,
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
                                                            255,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        105,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                270,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            110,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        279,
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
                                                            295,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        121,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 122,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                48,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `upmost_match`,
                                                            token_idx: TokenIdx(
                                                                50,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                61,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `eff_holes`,
                                                            token_idx: TokenIdx(
                                                                63,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `lower_excess`,
                                                            token_idx: TokenIdx(
                                                                69,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                86,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match`,
                                                            token_idx: TokenIdx(
                                                                106,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match_dp`,
                                                            token_idx: TokenIdx(
                                                                115,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match_dp_y`,
                                                            token_idx: TokenIdx(
                                                                123,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `upmost_match_dp_y`,
                                                            token_idx: TokenIdx(
                                                                129,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                139,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                153,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `rel_upmost_match_end`,
                                                            token_idx: TokenIdx(
                                                                179,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                278,
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
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
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
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
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
                                                            49,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    302,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            51,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    302,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `upmost_match`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            62,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    302,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            64,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    302,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `eff_holes`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            70,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    302,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `lower_excess`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            87,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            107,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `bottom1_match`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            116,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `bottom1_match_dp`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            124,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `bottom1_match_dp_y`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            130,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `upmost_match_dp_y`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            140,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `others`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            154,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    178,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 11,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            180,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    229,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `rel_upmost_match_end`,
                                                            pattern_symbol_idx: 12,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            279,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    295,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 13,
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
                                                expr: 123,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    123,
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
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            307,
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
                                                            311,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            308,
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
                                                            312,
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
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    305,
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
                                                                306,
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
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                304,
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
                                                        306,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                309,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                310,
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
                                                313,
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
                                                            Expr::Prefix {
                                                                opr: Tilde,
                                                                opr_token_idx: TokenIdx(
                                                                    307,
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
                                                                    311,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    308,
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
                                                                    312,
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
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            305,
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
                                                                        306,
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
                                                        317,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        318,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            319,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        320,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        321,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        323,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        324,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            325,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        327,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        326,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        328,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        329,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            330,
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
                                                            314,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 0,
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
                                                                316,
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
                                                            322,
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
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                315,
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
                                                Let,
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
                                                        `dp`,
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
                                                data: [
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        modifier: Pure,
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            316,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    331,
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
                                body: Some(
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
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            336,
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
                                                            340,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            337,
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
                                                            341,
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
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    334,
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
                                                                335,
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
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                333,
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
                                                        335,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                338,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                339,
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
                                                            Expr::Prefix {
                                                                opr: Tilde,
                                                                opr_token_idx: TokenIdx(
                                                                    336,
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
                                                                    340,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    337,
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
                                                                    341,
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
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            334,
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
                                                                        335,
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
                                                        346,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        347,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            348,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        349,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        350,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        352,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        357,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        353,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            354,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        356,
                                                    ),
                                                    opd: 3,
                                                },
                                                Expr::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        355,
                                                    ),
                                                    ropd: 5,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        361,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        365,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 7,
                                                    dot_token_idx: TokenIdx(
                                                        362,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            363,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 8,
                                                    dot_token_idx: TokenIdx(
                                                        366,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            367,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 9,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        364,
                                                    ),
                                                    ropd: 10,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        360,
                                                    ),
                                                    item: 11,
                                                    rpar_token_idx: TokenIdx(
                                                        368,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 12,
                                                    dot_token_idx: TokenIdx(
                                                        369,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            370,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        371,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        13..13,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        372,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        374,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 13,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        373,
                                                    ),
                                                    ropd: 14,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        376,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 16,
                                                    dot_token_idx: TokenIdx(
                                                        377,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            378,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 17,
                                                    dot_token_idx: TokenIdx(
                                                        379,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            380,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        381,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        18..18,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        382,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        384,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 18,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        383,
                                                    ),
                                                    ropd: 19,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        388,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 21,
                                                    dot_token_idx: TokenIdx(
                                                        389,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `line_segment_sketch`,
                                                        token_idx: TokenIdx(
                                                            390,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 22,
                                                    dot_token_idx: TokenIdx(
                                                        391,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            392,
                                                        ),
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        396,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 24,
                                                    dot_token_idx: TokenIdx(
                                                        397,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            398,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        399,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        25..25,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        400,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 23,
                                                    dot_token_idx: TokenIdx(
                                                        393,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_point`,
                                                        token_idx: TokenIdx(
                                                            394,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        395,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        25..26,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        401,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `relative_end`,
                                                    token_idx: TokenIdx(
                                                        403,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 27,
                                                    dot_token_idx: TokenIdx(
                                                        404,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            405,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        407,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 28,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        406,
                                                    ),
                                                    ropd: 29,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        409,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 31,
                                                    dot_token_idx: TokenIdx(
                                                        410,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            411,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        412,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        32..32,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        413,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 32,
                                                    dot_token_idx: TokenIdx(
                                                        414,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            415,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        408,
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
                                                            359,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        15,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            343,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 0,
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
                                                                345,
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
                                                                351,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            6,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        358,
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
                                                            375,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        20,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            385,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr_idx: 1,
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
                                                                387,
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
                                                            402,
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
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                344,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `relative_end`,
                                                            token_idx: TokenIdx(
                                                                386,
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
                                                Let,
                                                Let,
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
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
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
                                                        modifier: Pure,
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            345,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    416,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `dp`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            387,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    416,
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
                                body: Some(
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