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
                                ImplicitParameterDeclPattern {
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
                            regular_parameters: [
                                RegularParameterDeclPattern {
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
                                            Expr::NonAssociatedEntityPath {
                                                entity_path_expr: 0,
                                                path: Some(
                                                    EntityPath::ModuleItem(
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
                                                items: ArenaIdxRange(
                                                    1..1,
                                                ),
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
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
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
                                                    modifier_keyword_group: Some(
                                                        Mut(
                                                            MutToken {
                                                                token_idx: TokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                        ),
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
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
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
                                                ExplicitParameter {
                                                    pattern_expr: 0,
                                                    ty: 3,
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
                                                    Expr::NonAssociatedEntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
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
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
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
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    6,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
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
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            9,
                                                                        ),
                                                                    },
                                                                ),
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
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
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
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 3,
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
                                            items: ArenaIdxRange(
                                                1..1,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                25,
                                            ),
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 0,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `len`,
                                            token_idx: TokenIdx(
                                                33,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                35,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                34,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                32,
                                            ),
                                            item: 5,
                                            rpar_token_idx: TokenIdx(
                                                36,
                                            ),
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 1,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
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
                                        Expr::Binary {
                                            lopd: 6,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                37,
                                            ),
                                            ropd: 7,
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 2,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                27,
                                            ),
                                            items: ArenaIdxRange(
                                                8..11,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    29,
                                                ),
                                                TokenIdx(
                                                    31,
                                                ),
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
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `isize`,
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
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
                                        data: [
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `len`,
                                                    token_idx: TokenIdx(
                                                        19,
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
                                                `len`,
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
                                        data: [
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    20,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            40,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `len`,
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
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Fn(
                    FnDefn {
                        path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                            implicit_parameters: [
                                ImplicitParameterDeclPattern {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `T`,
                                            token_idx: TokenIdx(
                                                43,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        44,
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ),
                                    },
                                },
                            ],
                            regular_parameters: [
                                RegularParameterDeclPattern {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            50,
                                        ),
                                    ),
                                    ty: 3,
                                },
                                RegularParameterDeclPattern {
                                    pattern: 1,
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            57,
                                        ),
                                    ),
                                    ty: 4,
                                },
                                RegularParameterDeclPattern {
                                    pattern: 2,
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            61,
                                        ),
                                    ),
                                    ty: 5,
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
                                                        path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NonAssociatedEntityPath {
                                                entity_path_expr: 0,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::BoxColonList {
                                                lbox_token_idx: TokenIdx(
                                                    51,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    52,
                                                ),
                                                items: ArenaIdxRange(
                                                    1..1,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    53,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    54,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                43,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::ExplicitApplication {
                                                function: 1,
                                                argument: 2,
                                            },
                                            Expr::NonAssociatedEntityPath {
                                                entity_path_expr: 1,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::NonAssociatedEntityPath {
                                                entity_path_expr: 2,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            62,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
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
                                                    modifier_keyword_group: Some(
                                                        Mut(
                                                            MutToken {
                                                                token_idx: TokenIdx(
                                                                    48,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                },
                                                PatternExpr::Ident {
                                                    modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `low`,
                                                        token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                    },
                                                },
                                                PatternExpr::Ident {
                                                    modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `high`,
                                                        token_idx: TokenIdx(
                                                            60,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Move,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                            Parameter,
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `arr`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `low`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `high`,
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Mut,
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
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        44,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    43,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Mut,
                                                    access_start: TokenIdx(
                                                        50,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Pure,
                                                    access_start: TokenIdx(
                                                        57,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                        ident: `low`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Pure,
                                                    access_start: TokenIdx(
                                                        61,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                        ident: `high`,
                                                        pattern_symbol_idx: 2,
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
                                                ExplicitParameter {
                                                    pattern_expr: 0,
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                ExplicitParameter {
                                                    pattern_expr: 1,
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                ExplicitParameter {
                                                    pattern_expr: 2,
                                                    ty: 5,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
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
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 4,
                                        },
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 5,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            22,
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
                                                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::NonAssociatedEntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            54,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                    Expr::NonAssociatedEntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::NonAssociatedEntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    45,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    58,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    62,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            48,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    49,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `low`,
                                                                token_idx: TokenIdx(
                                                                    56,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `high`,
                                                                token_idx: TokenIdx(
                                                                    60,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Move,
                                                        Pure,
                                                        Pure,
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                    Parameter,
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
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `arr`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `low`,
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `high`,
                                                            2,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Mut,
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
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                44,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            43,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                50,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                57,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `low`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                61,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `high`,
                                                                pattern_symbol_idx: 2,
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
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 3,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 1,
                                                            ty: 4,
                                                        },
                                                        ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 2,
                                                            ty: 5,
                                                        },
                                                        ArenaIdxRange(
                                                            3..4,
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
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 5,
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
                                                    path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                66,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `low`,
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                68,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 0,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                67,
                                            ),
                                            ropd: 1,
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 0,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::partition`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                75,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                77,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `low`,
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                79,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 3,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                74,
                                            ),
                                            items: ArenaIdxRange(
                                                4..7,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    76,
                                                ),
                                                TokenIdx(
                                                    78,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                80,
                                            ),
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 1,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `p`,
                                            token_idx: TokenIdx(
                                                87,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                89,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                83,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                85,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `low`,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 9,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                88,
                                            ),
                                            ropd: 10,
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 8,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                82,
                                            ),
                                            items: ArenaIdxRange(
                                                11..14,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    84,
                                                ),
                                                TokenIdx(
                                                    86,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                90,
                                            ),
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 2,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `p`,
                                            token_idx: TokenIdx(
                                                95,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                97,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                93,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 16,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                96,
                                            ),
                                            ropd: 17,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                99,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 15,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                92,
                                            ),
                                            items: ArenaIdxRange(
                                                18..21,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    94,
                                                ),
                                                TokenIdx(
                                                    98,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                100,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `partition`,
                                                    token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::partition`, `Fn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        81,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
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
                                                    70,
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
                                                        72,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 7,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 14,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 21,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        65,
                                                    ),
                                                },
                                                condition: Ok(
                                                    2,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                69,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..3,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `p`,
                                                    token_idx: TokenIdx(
                                                        71,
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
                                                `p`,
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
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Pure,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: Pure,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `high`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    72,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            101,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `p`,
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
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 14,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 21,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 22,
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
                        path: FugitivePath(`quick_sort::partition`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`quick_sort::partition`, `Fn`),
                            implicit_parameters: [
                                ImplicitParameterDeclPattern {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `T`,
                                            token_idx: TokenIdx(
                                                104,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        105,
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ),
                                    },
                                },
                            ],
                            regular_parameters: [
                                RegularParameterDeclPattern {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            111,
                                        ),
                                    ),
                                    ty: 3,
                                },
                                RegularParameterDeclPattern {
                                    pattern: 1,
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            118,
                                        ),
                                    ),
                                    ty: 4,
                                },
                                RegularParameterDeclPattern {
                                    pattern: 2,
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            122,
                                        ),
                                    ),
                                    ty: 5,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 6,
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
                                                        path: FugitivePath(`quick_sort::partition`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NonAssociatedEntityPath {
                                                entity_path_expr: 0,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::BoxColonList {
                                                lbox_token_idx: TokenIdx(
                                                    112,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    113,
                                                ),
                                                items: ArenaIdxRange(
                                                    1..1,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    115,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                104,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::ExplicitApplication {
                                                function: 1,
                                                argument: 2,
                                            },
                                            Expr::NonAssociatedEntityPath {
                                                entity_path_expr: 1,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::NonAssociatedEntityPath {
                                                entity_path_expr: 2,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::NonAssociatedEntityPath {
                                                entity_path_expr: 3,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            106,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            123,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            126,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
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
                                                    modifier_keyword_group: Some(
                                                        Mut(
                                                            MutToken {
                                                                token_idx: TokenIdx(
                                                                    109,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            110,
                                                        ),
                                                    },
                                                },
                                                PatternExpr::Ident {
                                                    modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `low`,
                                                        token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                },
                                                PatternExpr::Ident {
                                                    modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `high`,
                                                        token_idx: TokenIdx(
                                                            121,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Move,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                            Parameter,
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `arr`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `low`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `high`,
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Mut,
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
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        105,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    104,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Mut,
                                                    access_start: TokenIdx(
                                                        111,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Pure,
                                                    access_start: TokenIdx(
                                                        118,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                        ident: `low`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Pure,
                                                    access_start: TokenIdx(
                                                        122,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                        ident: `high`,
                                                        pattern_symbol_idx: 2,
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
                                                ExplicitParameter {
                                                    pattern_expr: 0,
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                ExplicitParameter {
                                                    pattern_expr: 1,
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                ExplicitParameter {
                                                    pattern_expr: 2,
                                                    ty: 5,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
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
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 4,
                                        },
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 5,
                                        },
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 6,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            62,
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
                                                                path: FugitivePath(`quick_sort::partition`, `Fn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::NonAssociatedEntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            115,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        104,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                    Expr::NonAssociatedEntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::NonAssociatedEntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::NonAssociatedEntityPath {
                                                        entity_path_expr: 3,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    106,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    119,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    123,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    126,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                            modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            109,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    110,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `low`,
                                                                token_idx: TokenIdx(
                                                                    117,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `high`,
                                                                token_idx: TokenIdx(
                                                                    121,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Move,
                                                        Pure,
                                                        Pure,
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                    Parameter,
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
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `arr`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `low`,
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `high`,
                                                            2,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Mut,
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
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                105,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            104,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                111,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                118,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `low`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                122,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `high`,
                                                                pattern_symbol_idx: 2,
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
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 3,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 1,
                                                            ty: 4,
                                                        },
                                                        ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 2,
                                                            ty: 5,
                                                        },
                                                        ArenaIdxRange(
                                                            3..4,
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
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 5,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 6,
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
                                                    path: FugitivePath(`quick_sort::partition`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                131,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 0,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 0,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                132,
                                            ),
                                            ropd: 1,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `low`,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                140,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                139,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                145,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                147,
                                            ),
                                            Literal::Bool(
                                                True,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                151,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 8,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                153,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                155,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 1,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 12,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                156,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                160,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                162,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 11,
                                            lbox_token_idx: TokenIdx(
                                                154,
                                            ),
                                            items: ArenaIdxRange(
                                                14..15,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                158,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 15,
                                            lbox_token_idx: TokenIdx(
                                                161,
                                            ),
                                            items: ArenaIdxRange(
                                                16..17,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                163,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 17,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                159,
                                            ),
                                            ropd: 18,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                165,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                167,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 20,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                166,
                                            ),
                                            ropd: 21,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                168,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                170,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 23,
                                            opr: AssignClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                169,
                                            ),
                                            ropd: 24,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                172,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                174,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                176,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                178,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 2,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 29,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                179,
                                            ),
                                            ropd: 30,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                183,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                185,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 28,
                                            lbox_token_idx: TokenIdx(
                                                177,
                                            ),
                                            items: ArenaIdxRange(
                                                31..32,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                181,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 32,
                                            lbox_token_idx: TokenIdx(
                                                184,
                                            ),
                                            items: ArenaIdxRange(
                                                33..34,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                186,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 26,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                173,
                                            ),
                                            ropd: 27,
                                        },
                                        Expr::Binary {
                                            lopd: 34,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ropd: 35,
                                        },
                                        Expr::Binary {
                                            lopd: 36,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                175,
                                            ),
                                            ropd: 37,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                188,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                190,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 39,
                                            opr: AssignClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ropd: 40,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                192,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                194,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 42,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                193,
                                            ),
                                            ropd: 43,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                199,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                203,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 3,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                207,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 4,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 46,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                204,
                                            ),
                                            ropd: 47,
                                        },
                                        Expr::Binary {
                                            lopd: 48,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                208,
                                            ),
                                            ropd: 49,
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 45,
                                            dot_token_idx: TokenIdx(
                                                200,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `swap`,
                                                token_idx: TokenIdx(
                                                    201,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                202,
                                            ),
                                            items: ArenaIdxRange(
                                                50..52,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    206,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                211,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                215,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 5,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                219,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::NonAssociatedEntityPath {
                                            entity_path_expr: 6,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 54,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                216,
                                            ),
                                            ropd: 55,
                                        },
                                        Expr::Binary {
                                            lopd: 56,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                220,
                                            ),
                                            ropd: 57,
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 53,
                                            dot_token_idx: TokenIdx(
                                                212,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `swap`,
                                                token_idx: TokenIdx(
                                                    213,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                214,
                                            ),
                                            items: ArenaIdxRange(
                                                58..60,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    218,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                222,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                223,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                9..15,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        180,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 22,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 41,
                                        },
                                        Stmt::Break {
                                            break_token: BreakToken {
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                        },
                                        Stmt::Eval {
                                            expr_idx: 52,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 10,
                                        },
                                        Stmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                            condition: Ok(
                                                19,
                                            ),
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            164,
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
                                        Stmt::Eval {
                                            expr_idx: 25,
                                        },
                                        Stmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    171,
                                                ),
                                            },
                                            condition: Ok(
                                                38,
                                            ),
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            187,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                },
                                                condition: Ok(
                                                    44,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                195,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                ElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    198,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            3..4,
                                                        ),
                                                    ),
                                                },
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
                                                        130,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 2,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    134,
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
                                                        137,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 5,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    141,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
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
                                                        144,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 6,
                                        },
                                        Stmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    146,
                                                ),
                                            },
                                            condition: Ok(
                                                7,
                                            ),
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            148,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    4..9,
                                                ),
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 60,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 61,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `pivot`,
                                                    token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                135,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `store_index`,
                                                    token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                142,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `last_index`,
                                                    token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Pure,
                                            Move,
                                            Move,
                                        ],
                                    },
                                    pattern_infos: [
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
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `pivot`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `store_index`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `last_index`,
                                                2,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Pure,
                                            Mut,
                                            Mut,
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
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Pure,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: Pure,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `high`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    130,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `pivot`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    137,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `store_index`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    144,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            224,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `last_index`,
                                                    pattern_symbol_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 6,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 10,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 22,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 25,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 41,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 52,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 60,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 61,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 62,
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