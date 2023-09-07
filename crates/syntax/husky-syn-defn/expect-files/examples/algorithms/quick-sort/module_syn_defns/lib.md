Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `T`,
                                            token_idx: TokenIdx(
                                                5,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        6,
                                                    ),
                                                ),
                                                1,
                                            ),
                                        ),
                                    },
                                },
                            ],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 1,
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            12,
                                        ),
                                    ),
                                    ty: 4,
                                },
                            ],
                            return_ty: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
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
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::BoxColonList {
                                                lbox_token_idx: TokenIdx(
                                                    13,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    14,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    15,
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    16,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: Some(
                                                        Mut(
                                                            MutToken {
                                                                token_idx: TokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            11,
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
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `arr`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Mut,
                                            ],
                                        },
                                    },
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Mut,
                                                    access_start: TokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
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
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Traits,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 4,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            13,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::MajorItem(
                                                    MajorItemSynNodePath::Fugitive(
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
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            13,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                        items: [],
                                                        rbox_token_idx: TokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    SynExpr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            16,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: SynPatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    11,
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
                                                        SynPatternSymbol::Atom(
                                                            1,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `arr`,
                                                            1,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Mut,
                                                    ],
                                                },
                                            },
                                            symbol_region: SynSymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSynSymbol {
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                6,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                syn_attrs: TemplateParameterSynAttrs {
                                                                    syn_attrs: [],
                                                                },
                                                                annotated_variance_token: None,
                                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                12,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        TemplateTypeParameter,
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
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: Traits,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 4,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
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
                                                22,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                23,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `len`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                25,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                26,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                29,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                31,
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
                                                34,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                36,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 6,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                35,
                                            ),
                                            ropd: 7,
                                        },
                                        SynExpr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                33,
                                            ),
                                            item: 8,
                                            rpar_token_idx: TokenIdx(
                                                37,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 9,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                38,
                                            ),
                                            ropd: 10,
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 3,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                28,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            30,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            32,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 11,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                40,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        27,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `isize`,
                                                    token_idx: TokenIdx(
                                                        39,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::isize`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    19,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
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
                                                        21,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 2,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 12,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `len`,
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
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            SynPatternSymbol::Atom(
                                                1,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `len`,
                                                1,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            None,
                                        ],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `T`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Mut,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    21,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            41,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `len`,
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
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 2,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 12,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 13,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `T`,
                                            token_idx: TokenIdx(
                                                44,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        45,
                                                    ),
                                                ),
                                                1,
                                            ),
                                        ),
                                    },
                                },
                            ],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 1,
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            51,
                                        ),
                                    ),
                                    ty: 4,
                                },
                                SpecificParameterObelisk::Regular {
                                    pattern: 2,
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            58,
                                        ),
                                    ),
                                    ty: 5,
                                },
                                SpecificParameterObelisk::Regular {
                                    pattern: 3,
                                    variables: ArenaIdxRange(
                                        4..5,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            62,
                                        ),
                                    ),
                                    ty: 6,
                                },
                            ],
                            return_ty: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
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
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::BoxColonList {
                                                lbox_token_idx: TokenIdx(
                                                    52,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    53,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    54,
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    55,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                44,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            46,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            59,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            63,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: Some(
                                                        Mut(
                                                            MutToken {
                                                                token_idx: TokenIdx(
                                                                    49,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            50,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `low`,
                                                        token_idx: TokenIdx(
                                                            57,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `high`,
                                                        token_idx: TokenIdx(
                                                            61,
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
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    2,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    3,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `arr`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `low`,
                                                    2,
                                                ),
                                            ],
                                            [
                                                (
                                                    `high`,
                                                    3,
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
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        45,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    44,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Mut,
                                                    access_start: TokenIdx(
                                                        51,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        58,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `low`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        62,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `high`,
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
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
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 3,
                                                    ty_expr_idx: 6,
                                                },
                                                ArenaIdxRange(
                                                    4..5,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Traits,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 6,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            23,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::MajorItem(
                                                    MajorItemSynNodePath::Fugitive(
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
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                        items: [],
                                                        rbox_token_idx: TokenIdx(
                                                            54,
                                                        ),
                                                    },
                                                    SynExpr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            55,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        44,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 3,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    46,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    59,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    63,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: SynPatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            49,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                            },
                                                        },
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `low`,
                                                                token_idx: TokenIdx(
                                                                    57,
                                                                ),
                                                            },
                                                        },
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `high`,
                                                                token_idx: TokenIdx(
                                                                    61,
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
                                                        SynPatternSymbol::Atom(
                                                            1,
                                                        ),
                                                        SynPatternSymbol::Atom(
                                                            2,
                                                        ),
                                                        SynPatternSymbol::Atom(
                                                            3,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `arr`,
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `low`,
                                                            2,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `high`,
                                                            3,
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
                                            symbol_region: SynSymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSynSymbol {
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                45,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                syn_attrs: TemplateParameterSynAttrs {
                                                                    syn_attrs: [],
                                                                },
                                                                annotated_variance_token: None,
                                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            44,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                51,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                58,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `low`,
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                62,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `high`,
                                                                pattern_symbol_idx: 3,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        TemplateTypeParameter,
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
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 3,
                                                            ty_expr_idx: 6,
                                                        },
                                                        ArenaIdxRange(
                                                            4..5,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: Traits,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 4,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 5,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 6,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
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
                                                67,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `low`,
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                69,
                                            ),
                                            inherited_symbol_idx: 4,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 1,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                68,
                                            ),
                                            ropd: 2,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::partition`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                76,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                78,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `low`,
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                80,
                                            ),
                                            inherited_symbol_idx: 4,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 4,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                75,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            77,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 6,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            79,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 7,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                81,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                84,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                86,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `low`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `p`,
                                            token_idx: TokenIdx(
                                                88,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                90,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 12,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                89,
                                            ),
                                            ropd: 13,
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 9,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                83,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 10,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            85,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 11,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            87,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 14,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                91,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                94,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `p`,
                                            token_idx: TokenIdx(
                                                96,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                98,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 18,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                97,
                                            ),
                                            ropd: 19,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                100,
                                            ),
                                            inherited_symbol_idx: 4,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 16,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                93,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 17,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            95,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 20,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            99,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 21,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                101,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                4..5,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `partition`,
                                                    token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::partition`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        82,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `quick_sort_aux`,
                                                    token_idx: TokenIdx(
                                                        92,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    71,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
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
                                                        73,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 8,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 15,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 22,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                },
                                                condition: Ok(
                                                    3,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                70,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                    ],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `p`,
                                                    token_idx: TokenIdx(
                                                        72,
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
                                            SynPatternSymbol::Atom(
                                                1,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `p`,
                                                1,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            None,
                                        ],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `T`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Mut,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    4,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `high`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    73,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            102,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `p`,
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
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 8,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 15,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 22,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 23,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`quick_sort::partition`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`quick_sort::partition`, `Fn`),
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `T`,
                                            token_idx: TokenIdx(
                                                105,
                                            ),
                                        },
                                        traits: Some(
                                            (
                                                ColonToken(
                                                    TokenIdx(
                                                        106,
                                                    ),
                                                ),
                                                1,
                                            ),
                                        ),
                                    },
                                },
                            ],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 1,
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            112,
                                        ),
                                    ),
                                    ty: 4,
                                },
                                SpecificParameterObelisk::Regular {
                                    pattern: 2,
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            119,
                                        ),
                                    ),
                                    ty: 5,
                                },
                                SpecificParameterObelisk::Regular {
                                    pattern: 3,
                                    variables: ArenaIdxRange(
                                        4..5,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            123,
                                        ),
                                    ),
                                    ty: 6,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonObelisk {
                                    expr: 7,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
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
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::BoxColonList {
                                                lbox_token_idx: TokenIdx(
                                                    113,
                                                ),
                                                colon_token_idx: TokenIdx(
                                                    114,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    115,
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `T`,
                                                token_idx: TokenIdx(
                                                    116,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                105,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            107,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            120,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            127,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: Some(
                                                        Mut(
                                                            MutToken {
                                                                token_idx: TokenIdx(
                                                                    110,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `arr`,
                                                        token_idx: TokenIdx(
                                                            111,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `low`,
                                                        token_idx: TokenIdx(
                                                            118,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `high`,
                                                        token_idx: TokenIdx(
                                                            122,
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
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    2,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    3,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `arr`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `low`,
                                                    2,
                                                ),
                                            ],
                                            [
                                                (
                                                    `high`,
                                                    3,
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
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        106,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    105,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Mut,
                                                    access_start: TokenIdx(
                                                        112,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        119,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `low`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        123,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `high`,
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
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
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 3,
                                                    ty_expr_idx: 6,
                                                },
                                                ArenaIdxRange(
                                                    4..5,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: Traits,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 6,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 7,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            63,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::MajorItem(
                                                    MajorItemSynNodePath::Fugitive(
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
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::BoxColonList {
                                                        lbox_token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                        colon_token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                        items: [],
                                                        rbox_token_idx: TokenIdx(
                                                            115,
                                                        ),
                                                    },
                                                    SynExpr::CurrentSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            116,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        105,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 3,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 4,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Ord`,
                                                                token_idx: TokenIdx(
                                                                    107,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    120,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    124,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `isize`,
                                                                token_idx: TokenIdx(
                                                                    127,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: SynPatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: Some(
                                                                Mut(
                                                                    MutToken {
                                                                        token_idx: TokenIdx(
                                                                            110,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            ident_token: IdentToken {
                                                                ident: `arr`,
                                                                token_idx: TokenIdx(
                                                                    111,
                                                                ),
                                                            },
                                                        },
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `low`,
                                                                token_idx: TokenIdx(
                                                                    118,
                                                                ),
                                                            },
                                                        },
                                                        SynPatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `high`,
                                                                token_idx: TokenIdx(
                                                                    122,
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
                                                        SynPatternSymbol::Atom(
                                                            1,
                                                        ),
                                                        SynPatternSymbol::Atom(
                                                            2,
                                                        ),
                                                        SynPatternSymbol::Atom(
                                                            3,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `arr`,
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `low`,
                                                            2,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `high`,
                                                            3,
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
                                            symbol_region: SynSymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSynSymbol {
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                106,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                syn_attrs: TemplateParameterSynAttrs {
                                                                    syn_attrs: [],
                                                                },
                                                                annotated_variance_token: None,
                                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `T`,
                                                                        token_idx: TokenIdx(
                                                                            105,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: Mut,
                                                            access_start: TokenIdx(
                                                                112,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `arr`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                119,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `low`,
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                123,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                ident: `high`,
                                                                pattern_symbol_idx: 3,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        TemplateTypeParameter,
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
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 3,
                                                            ty_expr_idx: 6,
                                                        },
                                                        ArenaIdxRange(
                                                            4..5,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: Traits,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 4,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 5,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 6,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 7,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
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
                                                132,
                                            ),
                                            inherited_symbol_idx: 4,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 1,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                133,
                                            ),
                                            ropd: 2,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `low`,
                                            token_idx: TokenIdx(
                                                139,
                                            ),
                                            inherited_symbol_idx: 3,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `low`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                141,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 4,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                140,
                                            ),
                                            ropd: 5,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `high`,
                                            token_idx: TokenIdx(
                                                146,
                                            ),
                                            inherited_symbol_idx: 4,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `high`,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                148,
                                            ),
                                            Literal::Bool(
                                                True,
                                            ),
                                        ),
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                150,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                152,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 9,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                151,
                                            ),
                                            ropd: 10,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                154,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                156,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 13,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                157,
                                            ),
                                            ropd: 14,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                161,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                163,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 12,
                                            lbox_token_idx: TokenIdx(
                                                155,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 15,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                159,
                                            ),
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 16,
                                            lbox_token_idx: TokenIdx(
                                                162,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 17,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                164,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 18,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                160,
                                            ),
                                            ropd: 19,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                166,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                168,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 21,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                167,
                                            ),
                                            ropd: 22,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                169,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                171,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 24,
                                            opr: AssignClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                170,
                                            ),
                                            ropd: 25,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                173,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                175,
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
                                                177,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                179,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 30,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                180,
                                            ),
                                            ropd: 31,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                184,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                186,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 29,
                                            lbox_token_idx: TokenIdx(
                                                178,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 32,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                182,
                                            ),
                                        },
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 33,
                                            lbox_token_idx: TokenIdx(
                                                185,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 34,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                187,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 27,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                174,
                                            ),
                                            ropd: 28,
                                        },
                                        SynExpr::Binary {
                                            lopd: 35,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                183,
                                            ),
                                            ropd: 36,
                                        },
                                        SynExpr::Binary {
                                            lopd: 37,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                176,
                                            ),
                                            ropd: 38,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                189,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                191,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 40,
                                            opr: AssignClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                190,
                                            ),
                                            ropd: 41,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                193,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                195,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 43,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                194,
                                            ),
                                            ropd: 44,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                200,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                204,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 47,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                205,
                                            ),
                                            ropd: 48,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `last_index`,
                                            token_idx: TokenIdx(
                                                208,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 50,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                209,
                                            ),
                                            ropd: 51,
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 46,
                                            dot_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `swap`,
                                                token_idx: TokenIdx(
                                                    202,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                203,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 49,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 52,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                211,
                                            ),
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `arr`,
                                            token_idx: TokenIdx(
                                                212,
                                            ),
                                            inherited_symbol_idx: 2,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `arr`,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                216,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 6,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 55,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                217,
                                            ),
                                            ropd: 56,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `pivot`,
                                            token_idx: TokenIdx(
                                                220,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 7,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 58,
                                            opr: As,
                                            opr_token_idx: TokenIdx(
                                                221,
                                            ),
                                            ropd: 59,
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 54,
                                            dot_token_idx: TokenIdx(
                                                213,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `swap`,
                                                token_idx: TokenIdx(
                                                    214,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                215,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 57,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            219,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 60,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                223,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `store_index`,
                                            token_idx: TokenIdx(
                                                224,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                10..16,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        134,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        158,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `usize`,
                                                    token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Eval {
                                            expr_idx: 23,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 42,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Break {
                                            break_token: BreakToken {
                                                token_idx: TokenIdx(
                                                    197,
                                                ),
                                            },
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 53,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 11,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    153,
                                                ),
                                            },
                                            condition: Ok(
                                                20,
                                            ),
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            165,
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
                                        SynStmt::Eval {
                                            expr_idx: 26,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    172,
                                                ),
                                            },
                                            condition: Ok(
                                                39,
                                            ),
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            188,
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
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                },
                                                condition: Ok(
                                                    45,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                196,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                SynElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            198,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    199,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    stmts: Ok(
                                                        ArenaIdxRange(
                                                            4..5,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    129,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
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
                                                        131,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 3,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    135,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
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
                                                        138,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 6,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
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
                                                        145,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 7,
                                        },
                                        SynStmt::While {
                                            while_token: WhileToken {
                                                token_idx: TokenIdx(
                                                    147,
                                                ),
                                            },
                                            condition: Ok(
                                                8,
                                            ),
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            149,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    5..10,
                                                ),
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 61,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 62,
                                            eol_semicolon: Ok(
                                                None,
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `pivot`,
                                                    token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                136,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `store_index`,
                                                    token_idx: TokenIdx(
                                                        137,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                143,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `last_index`,
                                                    token_idx: TokenIdx(
                                                        144,
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
                                            SynPatternSymbol::Atom(
                                                1,
                                            ),
                                            SynPatternSymbol::Atom(
                                                2,
                                            ),
                                            SynPatternSymbol::Atom(
                                                3,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `pivot`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `store_index`,
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                `last_index`,
                                                3,
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `T`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: Mut,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    4,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `high`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    131,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            225,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `pivot`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    138,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            225,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `store_index`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    145,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            225,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `last_index`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 3,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 6,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 7,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 11,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 23,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 26,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 42,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 53,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 61,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 62,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 63,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [],
                            return_ty: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
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
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
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
                                    symbol_region: SynSymbolRegion {
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
                            16,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::MajorItem(
                                                    MajorItemSynNodePath::Fugitive(
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
                                            principal_item_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: SynPatternExprRegion {
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
                                            symbol_region: SynSymbolRegion {
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
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
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
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                236,
                                            ),
                                            items: [],
                                            rbox_token_idx: TokenIdx(
                                                237,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            item_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::ExplicitApplication {
                                            function_expr_idx: 1,
                                            argument_expr_idx: 2,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                241,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    4,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                243,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    65,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                245,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                248,
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
                                                247,
                                            ),
                                            opd: 7,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                250,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                252,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    99,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                254,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                256,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    83,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                258,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    782,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                260,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                240,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            242,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            244,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 6,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            246,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 8,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            249,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 9,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            251,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 10,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            253,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 11,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            255,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 12,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            257,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 13,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            259,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 14,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                261,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `i32`,
                                                    token_idx: TokenIdx(
                                                        238,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    232,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
                                                    pattern_expr_idx: 1,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    colon_token: Ok(
                                                        Some(
                                                            ColonToken(
                                                                TokenIdx(
                                                                    235,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    ty: Some(
                                                        3,
                                                    ),
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        239,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 15,
                                        },
                                    ],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                233,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `v`,
                                                    token_idx: TokenIdx(
                                                        234,
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
                                            SynPatternSymbol::Atom(
                                                1,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `v`,
                                                1,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Mut,
                                        ],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    235,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            262,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `v`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            LetVariables {
                                                pattern: 1,
                                                ty: 3,
                                            },
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [
                                    SynExprRoot {
                                        kind: LetStmtType,
                                        expr_idx: 3,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 15,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 16,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [],
                            return_ty: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
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
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
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
                                    symbol_region: SynSymbolRegion {
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
                            8,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::MajorItem(
                                                    MajorItemSynNodePath::Fugitive(
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
                                            principal_item_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: SynPatternExprRegion {
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
                                            symbol_region: SynSymbolRegion {
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
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
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
                                                274,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "beach",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                276,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "hotel",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                278,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "airplane",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                280,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "car",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                282,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "house",
                                                },
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                284,
                                            ),
                                            Literal::String(
                                                StringLiteral {
                                                    data: "art",
                                                },
                                            ),
                                        ),
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                273,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 1,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            275,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            277,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            279,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            281,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            283,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 6,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                285,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    269,
                                                ),
                                            },
                                            let_variables_pattern: Ok(
                                                LetVariableObelisk {
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
                                                        272,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 7,
                                        },
                                    ],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: Some(
                                                    Mut(
                                                        MutToken {
                                                            token_idx: TokenIdx(
                                                                270,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `strs`,
                                                    token_idx: TokenIdx(
                                                        271,
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
                                            SynPatternSymbol::Atom(
                                                1,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `strs`,
                                                1,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Mut,
                                        ],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    272,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            286,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSynSymbolVariant::LetVariable {
                                                    ident: `strs`,
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
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 7,
                                    },
                                    SynExprRoot {
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
    ],
)