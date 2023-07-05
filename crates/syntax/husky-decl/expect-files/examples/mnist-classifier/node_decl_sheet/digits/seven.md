Ok(
    NodeDeclSheet {
        [salsa id]: 32,
        decls: [
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Val(
                            ValNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 40,
                                colon_token: Ok(
                                    Some(
                                        ColonToken(
                                            TokenIdx(
                                                6,
                                            ),
                                        ),
                                    ),
                                ),
                                val_ty: Ok(
                                    Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                ),
                                eq_token: Ok(
                                    EqToken(
                                        TokenIdx(
                                            8,
                                        ),
                                    ),
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                                kind: VarType,
                                                expr_idx: 0,
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
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Fn(
                            FnNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 41,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                19,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [
                                            ExplicitParameterDecl::Regular {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        21,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                24,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                25,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                28,
                                            ),
                                        },
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                                        22,
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
                                                        26,
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
                                                                23,
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
                                                                27,
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
                                                                20,
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
                                                            21,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Val(
                            ValNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 42,
                                colon_token: Ok(
                                    Some(
                                        ColonToken(
                                            TokenIdx(
                                                49,
                                            ),
                                        ),
                                    ),
                                ),
                                val_ty: Ok(
                                    Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                ),
                                eq_token: Ok(
                                    EqToken(
                                        TokenIdx(
                                            51,
                                        ),
                                    ),
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `FermiMatchResult`,
                                                            token_idx: TokenIdx(
                                                                50,
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
                                                kind: VarType,
                                                expr_idx: 0,
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
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Fn(
                            FnNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 43,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                64,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [
                                            ExplicitParameterDecl::Regular {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        66,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                69,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                70,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                73,
                                            ),
                                        },
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
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
                                                        67,
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
                                                        71,
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
                                                                68,
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
                                                                72,
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
                                                                65,
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
                                                            66,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Fn(
                            FnNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 44,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                107,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [
                                            ExplicitParameterDecl::Regular {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        109,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                112,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                113,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                116,
                                            ),
                                        },
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                                        110,
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
                                                        114,
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
                                                                111,
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
                                                                115,
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
                                                                108,
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
                                                            109,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Val(
                            ValNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 45,
                                colon_token: Ok(
                                    Some(
                                        ColonToken(
                                            TokenIdx(
                                                165,
                                            ),
                                        ),
                                    ),
                                ),
                                val_ty: Ok(
                                    Some(
                                        FormTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                ),
                                eq_token: Ok(
                                    EqToken(
                                        TokenIdx(
                                            168,
                                        ),
                                    ),
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
                                                            path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                                        166,
                                                    ),
                                                    opd: 0,
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `MnistLabel`,
                                                            token_idx: TokenIdx(
                                                                167,
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
                                                kind: VarType,
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)