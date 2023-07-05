Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Fn(
                    FnDefn {
                        path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                            implicit_parameters: [
                                ImplicitParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `T`,
                                            token_idx: TokenIdx(
                                                4,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ),
                                    },
                                },
                            ],
                            explicit_parameters: [
                                ExplicitParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            11,
                                        ),
                                    ),
                                    ty: 3,
                                },
                            ],
                            return_ty: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Fugitive(
                                                FugitiveNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
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
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::BoxColonList {
                                                lbox_token_idx: TokenIdx(
                                                    12,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    13,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    14,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    15,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::ExplicitApplication {
                                                function: 1,
                                                argument: 2,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
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
                                                    symbol_modifier_keyword_group: Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                9,
                                                            ),
                                                        },
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Move,
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
                                                    `arr`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Mut,
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
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Mut,
                                                    access_start: TokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Traits,
                                            expr_idx: 0,
                                        },
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            12,
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
                                                                path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
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
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            12,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            13,
                                                        ),
                                                        items: [],
                                                        rbox_token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            15,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    6,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
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
                                                            symbol_modifier_keyword_group: Mut(
                                                                MutToken {
                                                                    token_idx: TokenIdx(
                                                                        9,
                                                                    ),
                                                                },
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Move,
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
                                                            `arr`,
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Mut,
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
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                5,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                11,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ImplicitTypeParameter,
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 3,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: Traits,
                                                    expr_idx: 0,
                                                },
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
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
                                                    path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                21,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                22,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `len`,
                                                token_idx: TokenIdx(
                                                    23,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                24,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                25,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                28,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                30,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        33,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 165,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                35,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                34,
                                            ),
                                            ropd: 6,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                32,
                                            ),
                                            item: 7,
                                            rpar_token_idx: TokenIdx(
                                                36,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 8,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                37,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 2,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                27,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            29,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            31,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 10,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                39,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..2,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `isize`,
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::isize`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    18,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                19,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        20,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 11,
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
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `T`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Mut,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `arr`,
                                                },
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
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 11,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 12,
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