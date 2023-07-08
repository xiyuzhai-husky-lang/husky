Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Fugitive(
                                                FugitiveNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Err(
                                                ExprError::Original(
                                                    NoRightOperandForBinaryOperator {
                                                        punctuation: Assign,
                                                        punctuation_token_idx: TokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `FermiMatchResult`,
                                                        token_idx: TokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            5,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::List {
                                            lbox_token_idx: TokenIdx(
                                                13,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                15,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                10,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 1,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            12,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: None,
                                                },
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
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        9,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
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
                                        kind: EvalExpr,
                                        expr_idx: 4,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 5,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Fugitive(
                                                FugitiveNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Err(
                                                ExprError::Original(
                                                    NoRightOperandForBinaryOperator {
                                                        punctuation: Assign,
                                                        punctuation_token_idx: TokenIdx(
                                                            21,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `FermiMatchResult`,
                                                        token_idx: TokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                            kind: ReturnType,
                                            expr_idx: 1,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            5,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::List {
                                            lbox_token_idx: TokenIdx(
                                                26,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                28,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                23,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 1,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            25,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                29,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        22,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        24,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `big_cc`,
                                                    token_idx: TokenIdx(
                                                        27,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
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
                                        kind: EvalExpr,
                                        expr_idx: 4,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 5,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 2,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Fugitive(
                                                FugitiveNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    37,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::Err(
                                                ExprError::Original(
                                                    NoRightOperandForBinaryOperator {
                                                        punctuation: Assign,
                                                        punctuation_token_idx: TokenIdx(
                                                            39,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            38,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                            kind: ReturnType,
                                            expr_idx: 2,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            83,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 0,
                                            be_token_idx: TokenIdx(
                                                42,
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
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 2,
                                            be_token_idx: TokenIdx(
                                                46,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 1,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                52,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    53,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                55,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                56,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    57,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                59,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 7,
                                            lbox_token_idx: TokenIdx(
                                                58,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 8,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                60,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 9,
                                            be_token_idx: TokenIdx(
                                                61,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 3,
                                                    variables: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                67,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    68,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                70,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 12,
                                            lbox_token_idx: TokenIdx(
                                                69,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 13,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                71,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `down_match`,
                                            token_idx: TokenIdx(
                                                73,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::Be {
                                            src: 15,
                                            be_token_idx: TokenIdx(
                                                74,
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
                                        Expr::CurrentSymbol {
                                            ident: `down_match`,
                                            token_idx: TokenIdx(
                                                79,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 17,
                                            dot_token_idx: TokenIdx(
                                                80,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    81,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                82,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                83,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 18,
                                            dot_token_idx: TokenIdx(
                                                84,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    85,
                                                ),
                                            },
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 20,
                                            dot_token_idx: TokenIdx(
                                                90,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    91,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                94,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    95,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 22,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                92,
                                            ),
                                            ropd: 23,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `higher_excess`,
                                            token_idx: TokenIdx(
                                                97,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                99,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 25,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                98,
                                            ),
                                            ropd: 26,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                101,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 28,
                                            dot_token_idx: TokenIdx(
                                                102,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    103,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                105,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 29,
                                            lbox_token_idx: TokenIdx(
                                                104,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 30,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                106,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 31,
                                            be_token_idx: TokenIdx(
                                                107,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 8,
                                                    variables: ArenaIdxRange(
                                                        8..9,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 6,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 33,
                                            dot_token_idx: TokenIdx(
                                                112,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    113,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                114,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                115,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                117,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 34,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                116,
                                            ),
                                            ropd: 35,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 7,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 37,
                                            dot_token_idx: TokenIdx(
                                                122,
                                            ),
                                            ident_token: IdentToken {
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
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 38,
                                            lbox_token_idx: TokenIdx(
                                                124,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 39,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                126,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `nine_match_refine_result`,
                                            token_idx: TokenIdx(
                                                128,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::Be {
                                            src: 41,
                                            be_token_idx: TokenIdx(
                                                129,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 10,
                                                    variables: ArenaIdxRange(
                                                        10..11,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 8,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 43,
                                            dot_token_idx: TokenIdx(
                                                133,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                136,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 44,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                135,
                                            ),
                                            ropd: 45,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 9,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 10,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 47,
                                            dot_token_idx: TokenIdx(
                                                141,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 48,
                                            dot_token_idx: TokenIdx(
                                                145,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    146,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 49,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                143,
                                            ),
                                            ropd: 50,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 11,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 52,
                                            dot_token_idx: TokenIdx(
                                                151,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                154,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 53,
                                            lbox_token_idx: TokenIdx(
                                                153,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 54,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                155,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                157,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::Be {
                                            src: 56,
                                            be_token_idx: TokenIdx(
                                                158,
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
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                161,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 58,
                                            dot_token_idx: TokenIdx(
                                                162,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                164,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                165,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 59,
                                            dot_token_idx: TokenIdx(
                                                166,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    167,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                169,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 60,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                168,
                                            ),
                                            ropd: 61,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                171,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                176,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 63,
                                            dot_token_idx: TokenIdx(
                                                172,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    173,
                                                ),
                                            },
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                175,
                                            ),
                                            opd: 64,
                                        },
                                        Expr::Binary {
                                            lopd: 65,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                174,
                                            ),
                                            ropd: 66,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 12,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 68,
                                            dot_token_idx: TokenIdx(
                                                179,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    180,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                182,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 69,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                181,
                                            ),
                                            ropd: 70,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 13,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                190,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 72,
                                            dot_token_idx: TokenIdx(
                                                187,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `top_k_row_right_mass_sum`,
                                                token_idx: TokenIdx(
                                                    188,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                189,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 73,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                191,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                193,
                                            ),
                                            current_symbol_idx: 14,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                195,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 75,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                194,
                                            ),
                                            ropd: 76,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                197,
                                            ),
                                            current_symbol_idx: 14,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                199,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 78,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                198,
                                            ),
                                            ropd: 79,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 15,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Nine`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 17,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Nine`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                14..25,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_zero`,
                                                    token_idx: TokenIdx(
                                                        41,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_six`,
                                                    token_idx: TokenIdx(
                                                        45,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        51,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `nine_match`,
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        89,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        93,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        111,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `nine_match_refine`,
                                                    token_idx: TokenIdx(
                                                        121,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `nine_match_refine`,
                                                    token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        144,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `nine_match_refine`,
                                                    token_idx: TokenIdx(
                                                        150,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `nine_match_refine`,
                                                    token_idx: TokenIdx(
                                                        178,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Subentity {
                                            parent: 14,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    202,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Nine`,
                                                    token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Nine`,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Subentity {
                                            parent: 16,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    205,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Nine`,
                                                    token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Nine`,
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
                                                    110,
                                                ),
                                            },
                                            condition: 36,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        120,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 40,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    127,
                                                ),
                                            },
                                            condition: 42,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    131,
                                                ),
                                            },
                                            condition: 46,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    137,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 11,
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
                                                        139,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 51,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    147,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        149,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 55,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    156,
                                                ),
                                            },
                                            condition: 57,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    160,
                                                ),
                                            },
                                            condition: 62,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                            },
                                            condition: 67,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    177,
                                                ),
                                            },
                                            condition: 71,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 14,
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
                                                EqToken(
                                                    TokenIdx(
                                                        185,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 74,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    192,
                                                ),
                                            },
                                            condition: 77,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                            condition: 80,
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    200,
                                                ),
                                            },
                                            result: 81,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    40,
                                                ),
                                            },
                                            condition: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    44,
                                                ),
                                            },
                                            condition: 3,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    48,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 2,
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
                                                        50,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 5,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    54,
                                                ),
                                            },
                                            condition: 10,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    63,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        65,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 14,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    72,
                                                ),
                                            },
                                            condition: 16,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    76,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        78,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 19,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    86,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        88,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 24,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    96,
                                                ),
                                            },
                                            condition: 27,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                },
                                                condition: Ok(
                                                    32,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                109,
                                                            ),
                                                        },
                                                    ),
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
                                            expr_idx: 82,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        43,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        47,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        49,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `down_match`,
                                                    token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        75,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `down_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        77,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `higher_excess`,
                                                    token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `nine_match_refine_result`,
                                                    token_idx: TokenIdx(
                                                        119,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `higher_excess`,
                                                    token_idx: TokenIdx(
                                                        138,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `upper_arc`,
                                                    token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        184,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
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
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `none`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `eff_holes`,
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                3,
                                            ),
                                        ],
                                        [
                                            (
                                                `down_match`,
                                                4,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                5,
                                            ),
                                        ],
                                        [
                                            (
                                                `down_match_dp_y`,
                                                6,
                                            ),
                                        ],
                                        [
                                            (
                                                `higher_excess`,
                                                7,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                8,
                                            ),
                                        ],
                                        [
                                            (
                                                `nine_match_refine_result`,
                                                9,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                10,
                                            ),
                                        ],
                                        [
                                            (
                                                `higher_excess`,
                                                11,
                                            ),
                                        ],
                                        [
                                            (
                                                `upper_arc`,
                                                12,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                13,
                                            ),
                                        ],
                                        [
                                            (
                                                `a`,
                                                14,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
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
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    44,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    48,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    50,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `eff_holes`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    63,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    65,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `down_match`,
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    76,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    78,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `down_match_dp_y`,
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    88,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `higher_excess`,
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    109,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    120,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `nine_match_refine_result`,
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    131,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 10,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    139,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `higher_excess`,
                                                    pattern_symbol_idx: 11,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    149,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `upper_arc`,
                                                    pattern_symbol_idx: 12,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    160,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 13,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    185,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            204,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `a`,
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
                                        kind: Condition,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 3,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 10,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 14,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 16,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 19,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 24,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 27,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 36,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 40,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 42,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 46,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 51,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 55,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 57,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 62,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 67,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 71,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 74,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 77,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 80,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 81,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 82,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 83,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Fn(
                    FnDefn {
                        path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                            implicit_parameters: [],
                            explicit_parameters: [
                                ExplicitParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            211,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 3,
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Fugitive(
                                                FugitiveNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    212,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    216,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            213,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            217,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            210,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                None,
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
                                                None,
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
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        211,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            8,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            212,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            216,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    213,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    217,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    210,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        None,
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
                                                        None,
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
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                211,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                222,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                223,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    224,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                225,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                226,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                228,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                229,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    230,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                232,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                231,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                233,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                234,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    235,
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
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    219,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        221,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    227,
                                                ),
                                            },
                                            condition: 5,
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
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            None,
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
                                            None,
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
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    221,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            236,
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
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 8,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Fn(
                    FnDefn {
                        path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                            implicit_parameters: [],
                            explicit_parameters: [
                                ExplicitParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            240,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 3,
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Fugitive(
                                                FugitiveNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    241,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    245,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            242,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            246,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            239,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                None,
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
                                                None,
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
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        240,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            14,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            241,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            245,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    242,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    246,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    239,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        None,
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
                                                        None,
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
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                240,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                251,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                252,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    253,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                254,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                255,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                257,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                258,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    259,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                261,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                260,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                263,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                264,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    265,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 7,
                                            dot_token_idx: TokenIdx(
                                                266,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    267,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                268,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                269,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                271,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 8,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                270,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                272,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                273,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    274,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 12,
                                            dot_token_idx: TokenIdx(
                                                275,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    276,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                277,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                278,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..4,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    248,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        250,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    256,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    262,
                                                ),
                                            },
                                            condition: 10,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 13,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        249,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            None,
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
                                            None,
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
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    250,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            279,
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
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 10,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 13,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 14,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)