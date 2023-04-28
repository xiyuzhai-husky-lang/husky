Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Val(
                            ValDefn {
                                path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                decl: ValDecl {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                        FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        13,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        15,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        10,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..4,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            12,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        16,
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
                                                        9,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                    ident: `upmost`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Val(
                            ValDefn {
                                path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                decl: ValDecl {
                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                    ast_idx: 51,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                19,
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
                                            21,
                                        ),
                                    ),
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                            20,
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
                                                        FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..3,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        30,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        23,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..5,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            25,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        31,
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
                                                        22,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        24,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        27,
                                                    ),
                                                    ident: `upmost`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        29,
                                                    ),
                                                    ident: `bottom1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                            FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Val(
                            ValDefn {
                                path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                decl: ValDecl {
                                    path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                            39,
                                                        ),
                                                        opd: 0,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            40,
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
                                                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 0,
                                                    be_token_idx: TokenIdx(
                                                        44,
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
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        53,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 3,
                                                    lbox_token_idx: TokenIdx(
                                                        52,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        54,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        56,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 6,
                                                    be_token_idx: TokenIdx(
                                                        57,
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
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 8,
                                                    dot_token_idx: TokenIdx(
                                                        63,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            64,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 10,
                                                    dot_token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            70,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 11,
                                                    dot_token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            74,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 12,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                    ropd: 13,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        76,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 15,
                                                    dot_token_idx: TokenIdx(
                                                        77,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            78,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        80,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 16,
                                                    lbox_token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        81,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 18,
                                                    be_token_idx: TokenIdx(
                                                        82,
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
                                                                85,
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
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 8,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                        91,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 21,
                                                    dot_token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            95,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 22,
                                                    dot_token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            99,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 20,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        86,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        23..27,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            90,
                                                        ),
                                                        TokenIdx(
                                                            92,
                                                        ),
                                                        TokenIdx(
                                                            96,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 27,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 9,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 29,
                                                    dot_token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                    ident_token: IdentToken {
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
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 30,
                                                    lbox_token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        31..32,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        110,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match`,
                                                    token_idx: TokenIdx(
                                                        114,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 33,
                                                    dot_token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            116,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        117,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        34..34,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        118,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match_dp`,
                                                    token_idx: TokenIdx(
                                                        122,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 35,
                                                    dot_token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 37,
                                                    dot_token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            130,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        38..38,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 38,
                                                    dot_token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            134,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 10,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 40,
                                                    dot_token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `others`,
                                                        token_idx: TokenIdx(
                                                            140,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 11,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 42,
                                                    dot_token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            144,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        146,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 43,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                    ropd: 44,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match`,
                                                    token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 46,
                                                    be_token_idx: TokenIdx(
                                                        149,
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
                                                        156,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `bottom1_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        153,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                    opd: 48,
                                                },
                                                Expr::Binary {
                                                    lopd: 49,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                    ropd: 50,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 12,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        160,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 52,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                    ropd: 53,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                161,
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
                                                        167,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        169,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 55,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        162,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        56..59,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            166,
                                                        ),
                                                        TokenIdx(
                                                            168,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 59,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        171,
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
                                                                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 62,
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
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 64,
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
                                                        65..65,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 63,
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
                                                        65..66,
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 21,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        209,
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
                                                        197,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `upmost_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        199,
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
                                                                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 68,
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
                                                        69..70,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 67,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        70..76,
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
                                                    opd: 76,
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
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 78,
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
                                                    lopd: 79,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    ropd: 80,
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
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                    owner: 85,
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
                                                    function: 84,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        86..89,
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
                                                    opd: 89,
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
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 91,
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
                                                    lopd: 92,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                    ropd: 93,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 30,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 95,
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
                                                    lopd: 96,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        253,
                                                    ),
                                                    ropd: 97,
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
                                                    owner: 99,
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
                                                    owner: 100,
                                                    lbox_token_idx: TokenIdx(
                                                        259,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        101..102,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        261,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 102,
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
                                                    self_argument: 103,
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
                                                        104..104,
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
                                                    lopd: 104,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        268,
                                                    ),
                                                    ropd: 105,
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
                                                    owner: 107,
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
                                                    owner: 108,
                                                    lbox_token_idx: TokenIdx(
                                                        274,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        109..110,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        276,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 110,
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
                                                    owner: 112,
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
                                                    owner: 113,
                                                    lbox_token_idx: TokenIdx(
                                                        284,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        114..115,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        286,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 115,
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
                                                    self_argument: 116,
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
                                                        117..117,
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
                                                    lopd: 117,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        293,
                                                    ),
                                                    ropd: 118,
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
                                                    lopd: 120,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        297,
                                                    ),
                                                    ropd: 121,
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
                                                        43,
                                                    ),
                                                    ident: `is_one`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        49,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        68,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        87,
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
                                                            88,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                89,
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
                                                        93,
                                                    ),
                                                    ident: `six_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        97,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        105,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        138,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        142,
                                                    ),
                                                    ident: `six_match_refined1`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        158,
                                                    ),
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        163,
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
                                                            164,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                165,
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
                                                        173,
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
                                                            174,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                175,
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
                                                        179,
                                                    ),
                                                    ident: `major_line_segment_sketch`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        193,
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
                                                            194,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                195,
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
                                                        201,
                                                    ),
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        222,
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
                                                            223,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Six`,
                                                            token_idx: TokenIdx(
                                                                224,
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
                                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                            FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                            152,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        51,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            157,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        54,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 60,
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            172,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        61,
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            221,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        82,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 28,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            102,
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
                                                                104,
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
                                                            111,
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
                                                                113,
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
                                                            119,
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
                                                                121,
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
                                                            125,
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
                                                                127,
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
                                                            135,
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
                                                                137,
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
                                                            141,
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
                                                                147,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            47,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        151,
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
                                                                178,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        66,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 77,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                214,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            81,
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
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            225,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        83,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            249,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        98,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            280,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        119,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            42,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            46,
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
                                                                48,
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
                                                            55,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        7,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            59,
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
                                                                61,
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
                                                            65,
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
                                                                67,
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
                                                                75,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            19,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        84,
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
                                                    expr_idx: 90,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                242,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            94,
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
                                                        106,
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
                                                            111,
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
                                                        122,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 123,
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
                                                                45,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `upmost_match`,
                                                            token_idx: TokenIdx(
                                                                47,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                58,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `eff_holes`,
                                                            token_idx: TokenIdx(
                                                                60,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `lower_excess`,
                                                            token_idx: TokenIdx(
                                                                66,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                83,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match`,
                                                            token_idx: TokenIdx(
                                                                103,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match_dp`,
                                                            token_idx: TokenIdx(
                                                                112,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `bottom1_match_dp_y`,
                                                            token_idx: TokenIdx(
                                                                120,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `upmost_match_dp_y`,
                                                            token_idx: TokenIdx(
                                                                126,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                136,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                150,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `rel_upmost_match_end`,
                                                            token_idx: TokenIdx(
                                                                177,
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
                                                            46,
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
                                                            48,
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
                                                            59,
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
                                                            61,
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
                                                            67,
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
                                                            84,
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
                                                            104,
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
                                                            113,
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
                                                            121,
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
                                                            127,
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
                                                            137,
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
                                                            151,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    176,
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
                                                            178,
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
                                                expr: 124,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    124,
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
                            FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                    ast_idx: 53,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                        comma_after_self_parameter: None,
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
                                                                    FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                                        FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                            FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                    ast_idx: 54,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                                        comma_after_self_parameter: None,
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
                                                                    FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                                                        FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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