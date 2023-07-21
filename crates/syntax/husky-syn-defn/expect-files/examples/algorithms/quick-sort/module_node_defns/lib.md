Ok(
    [
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: FnNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 30,
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                3,
                                            ),
                                        ),
                                        generic_parameters: [
                                            GenericParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: GenericParameterDeclPatternVariant::Type {
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
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                7,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            parenic_parameter_decl_list: Ok(
                                SelfParameterAndExplicitParameters {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            8,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenic_parameters: [
                                        SpecificParameterDecl::Regular {
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
                                    commas: [],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            16,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                None,
                            ),
                            return_ty: Ok(
                                None,
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: SynExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
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
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::BoxColonList {
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
                                            SynExpr::CurrentSymbol {
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
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
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
                                                    symbol_modifier_keyword_group: Some(
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
                        expr_region: SynExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
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
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::BoxColonList {
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
                                                    SynExpr::CurrentSymbol {
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
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
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
                                                            symbol_modifier_keyword_group: Some(
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
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
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                21,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
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
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                28,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                30,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::CurrentSymbol {
                                            ident: `len`,
                                            token_idx: TokenIdx(
                                                33,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                35,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                34,
                                            ),
                                            ropd: 6,
                                        },
                                        SynExpr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                32,
                                            ),
                                            item: 7,
                                            rpar_token_idx: TokenIdx(
                                                36,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 8,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                37,
                                            ),
                                            ropd: 9,
                                        },
                                        SynExpr::FunctionApplicationOrCall {
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
                                        SynExpr::Block {
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
                                                symbol_modifier_keyword_group: None,
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
                                                `len`,
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
                                                modifier: None,
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
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: FnNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 31,
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                42,
                                            ),
                                        ),
                                        generic_parameters: [
                                            GenericParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: GenericParameterDeclPatternVariant::Type {
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
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                46,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            parenic_parameter_decl_list: Ok(
                                SelfParameterAndExplicitParameters {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            47,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenic_parameters: [
                                        SpecificParameterDecl::Regular {
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
                                        SpecificParameterDecl::Regular {
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
                                        SpecificParameterDecl::Regular {
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
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                55,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                59,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            63,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                None,
                            ),
                            return_ty: Ok(
                                None,
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            64,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: SynExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
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
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::BoxColonList {
                                                lbox_token_idx: TokenIdx(
                                                    51,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    52,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    53,
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
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
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
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
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            62,
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
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Ident {
                                                    symbol_modifier_keyword_group: Some(
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
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `low`,
                                                        token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                    },
                                                },
                                                PatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
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
                                                None,
                                                None,
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
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        57,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `low`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        61,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 1,
                                                    ty_expr_idx: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 2,
                                                    ty_expr_idx: 5,
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
                        expr_region: SynExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
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
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        items: [],
                                                        rbox_token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                    SynExpr::CurrentSymbol {
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
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
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
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    45,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    58,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    62,
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
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            symbol_modifier_keyword_group: Some(
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
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `low`,
                                                                token_idx: TokenIdx(
                                                                    56,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
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
                                                        None,
                                                        None,
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
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                57,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `low`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                61,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 3,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 1,
                                                            ty_expr_idx: 4,
                                                        },
                                                        ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 2,
                                                            ty_expr_idx: 5,
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
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
                                        SynExpr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                66,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `low`,
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                68,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 0,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                67,
                                            ),
                                            ropd: 1,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::partition`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                75,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                77,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `low`,
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                79,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 3,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                74,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            76,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            78,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 6,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                80,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                83,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                85,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `low`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `p`,
                                            token_idx: TokenIdx(
                                                87,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                89,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 11,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                88,
                                            ),
                                            ropd: 12,
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 8,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                82,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 9,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            84,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 10,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            86,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 13,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                90,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                93,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `p`,
                                            token_idx: TokenIdx(
                                                95,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                97,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 17,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                96,
                                            ),
                                            ropd: 18,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                99,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 15,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                92,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 16,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            94,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 19,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            98,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 20,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                100,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `partition`,
                                                    token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::partition`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        81,
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
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                symbol_modifier_keyword_group: None,
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
                                                `p`,
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
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `high`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: None,
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
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::partition`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: FnNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`quick_sort::partition`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 32,
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                103,
                                            ),
                                        ),
                                        generic_parameters: [
                                            GenericParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: GenericParameterDeclPatternVariant::Type {
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
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                107,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            parenic_parameter_decl_list: Ok(
                                SelfParameterAndExplicitParameters {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            108,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenic_parameters: [
                                        SpecificParameterDecl::Regular {
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
                                        SpecificParameterDecl::Regular {
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
                                        SpecificParameterDecl::Regular {
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
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                116,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                120,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            124,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            125,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 6,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            127,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: SynExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
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
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::BoxColonList {
                                                lbox_token_idx: TokenIdx(
                                                    112,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    113,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
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
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
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
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            106,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            123,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            126,
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
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternExpr::Ident {
                                                    symbol_modifier_keyword_group: Some(
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
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `low`,
                                                        token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                },
                                                PatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
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
                                                None,
                                                None,
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
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        118,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `low`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        122,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 1,
                                                    ty_expr_idx: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 2,
                                                    ty_expr_idx: 5,
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
                        expr_region: SynExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
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
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                        items: [],
                                                        rbox_token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                    },
                                                    SynExpr::CurrentSymbol {
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
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 3,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
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
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    106,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    119,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    123,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    126,
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
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            symbol_modifier_keyword_group: Some(
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
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `low`,
                                                                token_idx: TokenIdx(
                                                                    117,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
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
                                                        None,
                                                        None,
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
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                118,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `low`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                122,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 3,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 1,
                                                            ty_expr_idx: 4,
                                                        },
                                                        ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 2,
                                                            ty_expr_idx: 5,
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
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
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
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                131,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 0,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                132,
                                            ),
                                            ropd: 1,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `low`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                140,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 3,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                139,
                                            ),
                                            ropd: 4,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                145,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                147,
                                            ),
                                            Literal::Bool(
                                                True,
                                            ),
                                        ),
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                151,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 8,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ropd: 9,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                153,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                155,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 12,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                156,
                                            ),
                                            ropd: 13,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                160,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                162,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 11,
                                            lbox_token_idx: TokenIdx(
                                                154,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 14,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                158,
                                            ),
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 15,
                                            lbox_token_idx: TokenIdx(
                                                161,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 16,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                163,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 17,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                159,
                                            ),
                                            ropd: 18,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                165,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                167,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 20,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                166,
                                            ),
                                            ropd: 21,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                168,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                170,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 23,
                                            opr: AssignClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                169,
                                            ),
                                            ropd: 24,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                172,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                174,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                176,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                178,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 29,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                179,
                                            ),
                                            ropd: 30,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                183,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                185,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 28,
                                            lbox_token_idx: TokenIdx(
                                                177,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 31,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                181,
                                            ),
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 32,
                                            lbox_token_idx: TokenIdx(
                                                184,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 33,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                186,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 26,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                173,
                                            ),
                                            ropd: 27,
                                        },
                                        SynExpr::Binary {
                                            lopd: 34,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ropd: 35,
                                        },
                                        SynExpr::Binary {
                                            lopd: 36,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                175,
                                            ),
                                            ropd: 37,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                188,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                190,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 39,
                                            opr: AssignClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ropd: 40,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                192,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                194,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 42,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                193,
                                            ),
                                            ropd: 43,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                199,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                203,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 46,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                204,
                                            ),
                                            ropd: 47,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                207,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 49,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                208,
                                            ),
                                            ropd: 50,
                                        },
                                        SynExpr::MethodApplicationOrCall {
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
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 48,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            206,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 51,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                211,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                215,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 54,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                216,
                                            ),
                                            ropd: 55,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                219,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 6,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 57,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                220,
                                            ),
                                            ropd: 58,
                                        },
                                        SynExpr::MethodApplicationOrCall {
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
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 56,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            218,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 59,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                222,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                223,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                9..15,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        180,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
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
                                                LetVariableDecls {
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
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `pivot`,
                                                    token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
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
                                                symbol_modifier_keyword_group: Some(
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
                                            None,
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
                                            None,
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
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `high`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: None,
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
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: FnNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 34,
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            parenic_parameter_decl_list: Err(
                                NodeDeclError::Original(
                                    OriginalNodeDeclError::ExpectedParameterDeclList(
                                        TokenStreamState {
                                            next_token_idx: TokenIdx(
                                                228,
                                            ),
                                            drained: false,
                                        },
                                    ),
                                ),
                            ),
                            curry_token: Ok(
                                None,
                            ),
                            return_ty: Ok(
                                None,
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            228,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: SynExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [],
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
                                    roots: [],
                                },
                            },
                        },
                        body: Some(
                            29,
                        ),
                        expr_region: SynExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [],
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
                                            roots: [],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExpr::Literal(
                                            TokenIdx(
                                                234,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    4,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                236,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    65,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                238,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                241,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    31,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                240,
                                            ),
                                            opd: 3,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                243,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                245,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    99,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                247,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                249,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    83,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                251,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    782,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                253,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                233,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 0,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            235,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 1,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            237,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            239,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            242,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            244,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 6,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            246,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 7,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 8,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            250,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 9,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            252,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 10,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                254,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `v`,
                                            token_idx: TokenIdx(
                                                257,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 12,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                256,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 13,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                258,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                264,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    31,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                263,
                                            ),
                                            opd: 15,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                266,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                268,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                270,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                272,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                274,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    4,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                276,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    65,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                278,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    83,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                280,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    99,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                282,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    782,
                                                ),
                                            ),
                                        ),
                                        SynExpr::CurrentSymbol {
                                            ident: `v`,
                                            token_idx: TokenIdx(
                                                260,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                262,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 16,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            265,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 17,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            267,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 18,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            269,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 19,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            271,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 20,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            273,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 21,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            275,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 22,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            277,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 23,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            279,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 24,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            281,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 25,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                283,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 26,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                261,
                                            ),
                                            ropd: 27,
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort`,
                                                    token_idx: TokenIdx(
                                                        255,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort`, `Fn`),
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
                                                    229,
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
                                                        232,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 11,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 14,
                                        },
                                        Stmt::Assert {
                                            assert_token: AssertToken {
                                                token_idx: TokenIdx(
                                                    259,
                                                ),
                                            },
                                            condition: 28,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                230,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `v`,
                                                    token_idx: TokenIdx(
                                                        231,
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
                                                `v`,
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
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    232,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            284,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `v`,
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
                                        expr_idx: 11,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 14,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 28,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 29,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: FnNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 36,
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            parenic_parameter_decl_list: Err(
                                NodeDeclError::Original(
                                    OriginalNodeDeclError::ExpectedParameterDeclList(
                                        TokenStreamState {
                                            next_token_idx: TokenIdx(
                                                288,
                                            ),
                                            drained: false,
                                        },
                                    ),
                                ),
                            ),
                            curry_token: Ok(
                                None,
                            ),
                            return_ty: Ok(
                                None,
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            288,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: SynExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [],
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
                                    roots: [],
                                },
                            },
                        },
                        body: Some(
                            19,
                        ),
                        expr_region: SynExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [],
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
                                            roots: [],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExpr::Literal(
                                            TokenIdx(
                                                294,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "beach",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                296,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "hotel",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                298,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "airplane",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                300,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "car",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                302,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "house",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                304,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "art",
                                                },
                                            ),
                                        ),
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                293,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 0,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            295,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 1,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            297,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            299,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            301,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            303,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                305,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `strs`,
                                            token_idx: TokenIdx(
                                                308,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 7,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                307,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 8,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                309,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                314,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "airplane",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                316,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "art",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                318,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "beach",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                320,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "car",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                322,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "hotel",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                324,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "house",
                                                },
                                            ),
                                        ),
                                        SynExpr::CurrentSymbol {
                                            ident: `strs`,
                                            token_idx: TokenIdx(
                                                311,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                313,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 10,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            315,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 11,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            317,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 12,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            319,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 13,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            321,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 14,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            323,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 15,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                325,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 16,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                312,
                                            ),
                                            ropd: 17,
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort`,
                                                    token_idx: TokenIdx(
                                                        306,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort`, `Fn`),
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
                                                    289,
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
                                                        292,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 6,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 9,
                                        },
                                        Stmt::Assert {
                                            assert_token: AssertToken {
                                                token_idx: TokenIdx(
                                                    310,
                                                ),
                                            },
                                            condition: 18,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                290,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `strs`,
                                                    token_idx: TokenIdx(
                                                        291,
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
                                                `strs`,
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
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    292,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            326,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `strs`,
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
                                        expr_idx: 6,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 9,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 18,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 19,
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