[
    SynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::FunctionFn(
                FnSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: FnSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                                disambiguator: 0,
                            },
                        },
                        template_parameter_obelisk_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterSyndicate {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            data: TemplateParameterSyndicateData::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: Some(
                                                    (
                                                        ColonRegionalToken(
                                                            RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        ),
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        parenate_parameter_obelisk_list: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [
                                    ParenateParameterSyndicate::Ordinary {
                                        syn_pattern_root: ParenateSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonRegionalToken(
                                            RegionalTokenIdx(
                                                12,
                                            ),
                                        ),
                                        ty: 4,
                                    },
                                ],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        17,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            None,
                        ),
                        return_ty: Ok(
                            None,
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::BoxColonList {
                                            lbox_regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                            colon_regional_token_idx: RegionalTokenIdx(
                                                14,
                                            ),
                                            items: [],
                                            rbox_regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `T`,
                                            regional_token_idx: RegionalTokenIdx(
                                                16,
                                            ),
                                            current_syn_symbol_idx: 1,
                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `T`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 2,
                                            argument_expr_idx: 3,
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `Ord`,
                                                    regional_token_idx: RegionalTokenIdx(
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
                                                symbol_modifier_tokens: Some(
                                                    Mut(
                                                        MutRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `arr`,
                                                    regional_token_idx: RegionalTokenIdx(
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
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: RegionalTokenIdx(
                                                    12,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 1,
                                                },
                                                ty_expr_idx: 4,
                                            },
                                            ArenaIdxRange(
                                                2..3,
                                            ),
                                        ),
                                    ],
                                },
                                syn_pattern_expr_roots: [
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 1,
                                    },
                                ],
                                syn_expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::Traits,
                                        syn_expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 4,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                            },
                        },
                    },
                    body_with_syn_expr_region: Some(
                        (
                            13,
                            SynExprRegion {
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
                                                                    path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::cmp::Ord`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::BoxColonList {
                                                            lbox_regional_token_idx: RegionalTokenIdx(
                                                                13,
                                                            ),
                                                            colon_regional_token_idx: RegionalTokenIdx(
                                                                14,
                                                            ),
                                                            items: [],
                                                            rbox_regional_token_idx: RegionalTokenIdx(
                                                                15,
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `T`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Ord`,
                                                                    regional_token_idx: RegionalTokenIdx(
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
                                                                symbol_modifier_tokens: Some(
                                                                    Mut(
                                                                        MutRegionalToken {
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `arr`,
                                                                    regional_token_idx: RegionalTokenIdx(
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
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `T`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Mut,
                                                                access_start: RegionalTokenIdx(
                                                                    12,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                            OrdinaryParenateParameter {
                                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                                    syn_pattern_expr_idx: 1,
                                                                },
                                                                ty_expr_idx: 4,
                                                            },
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [
                                                    SynPatternExprRoot {
                                                        kind: SynPatternExprRootKind::Parenate,
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                ],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Traits,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ExplicitParameterType,
                                                        syn_expr_idx: 4,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::MethodApplicationOrCall {
                                                self_argument: 1,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `len`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                },
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::CurrentSynSymbol {
                                                ident: `len`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    18,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Binary {
                                                lopd: 6,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                ropd: 7,
                                            },
                                            SynExprData::Bracketed {
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                item: 8,
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 9,
                                                opr: As,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    20,
                                                ),
                                                ropd: 10,
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 3,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 4,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                12,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 5,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                14,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 11,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    22,
                                                ),
                                            },
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..3,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `quick_sort_aux`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `isize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            21,
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
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        1,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 1,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 2,
                                            },
                                            SynStmtData::Eval {
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
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `len`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            2,
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
                                        inherited_syn_symbol_arena: Arena {
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
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                23,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 12,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 13,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    SynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::FunctionFn(
                FnSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: FnSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                disambiguator: 0,
                            },
                        },
                        template_parameter_obelisk_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterSyndicate {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            data: TemplateParameterSyndicateData::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                },
                                                traits: Some(
                                                    (
                                                        ColonRegionalToken(
                                                            RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        ),
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        parenate_parameter_obelisk_list: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [
                                    ParenateParameterSyndicate::Ordinary {
                                        syn_pattern_root: ParenateSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonRegionalToken(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                        ty: 4,
                                    },
                                    ParenateParameterSyndicate::Ordinary {
                                        syn_pattern_root: ParenateSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon: ColonRegionalToken(
                                            RegionalTokenIdx(
                                                18,
                                            ),
                                        ),
                                        ty: 5,
                                    },
                                    ParenateParameterSyndicate::Ordinary {
                                        syn_pattern_root: ParenateSynPatternExprRoot {
                                            syn_pattern_expr_idx: 3,
                                        },
                                        variables: ArenaIdxRange(
                                            4..5,
                                        ),
                                        colon: ColonRegionalToken(
                                            RegionalTokenIdx(
                                                22,
                                            ),
                                        ),
                                        ty: 6,
                                    },
                                ],
                                commas: [
                                    CommaRegionalToken(
                                        RegionalTokenIdx(
                                            16,
                                        ),
                                    ),
                                    CommaRegionalToken(
                                        RegionalTokenIdx(
                                            20,
                                        ),
                                    ),
                                ],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        24,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            None,
                        ),
                        return_ty: Ok(
                            None,
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::BoxColonList {
                                            lbox_regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                            colon_regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                            items: [],
                                            rbox_regional_token_idx: RegionalTokenIdx(
                                                14,
                                            ),
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `T`,
                                            regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                            current_syn_symbol_idx: 1,
                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `T`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 2,
                                            argument_expr_idx: 3,
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 3,
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
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `Ord`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::Ord`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `isize`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::isize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `isize`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        23,
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
                                                symbol_modifier_tokens: Some(
                                                    Mut(
                                                        MutRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                9,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `arr`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_tokens: None,
                                                ident_token: IdentRegionalToken {
                                                    ident: `low`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_tokens: None,
                                                ident_token: IdentRegionalToken {
                                                    ident: `high`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
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
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    5,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: RegionalTokenIdx(
                                                    11,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                    ident: `arr`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: RegionalTokenIdx(
                                                    18,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                    ident: `low`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: RegionalTokenIdx(
                                                    22,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 1,
                                                },
                                                ty_expr_idx: 4,
                                            },
                                            ArenaIdxRange(
                                                2..3,
                                            ),
                                        ),
                                        (
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 2,
                                                },
                                                ty_expr_idx: 5,
                                            },
                                            ArenaIdxRange(
                                                3..4,
                                            ),
                                        ),
                                        (
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 3,
                                                },
                                                ty_expr_idx: 6,
                                            },
                                            ArenaIdxRange(
                                                4..5,
                                            ),
                                        ),
                                    ],
                                },
                                syn_pattern_expr_roots: [
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 1,
                                    },
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 2,
                                    },
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 3,
                                    },
                                ],
                                syn_expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::Traits,
                                        syn_expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 4,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 5,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 6,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                            },
                        },
                    },
                    body_with_syn_expr_region: Some(
                        (
                            23,
                            SynExprRegion {
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
                                                                    path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::cmp::Ord`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::BoxColonList {
                                                            lbox_regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                            colon_regional_token_idx: RegionalTokenIdx(
                                                                13,
                                                            ),
                                                            items: [],
                                                            rbox_regional_token_idx: RegionalTokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                15,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `T`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::num::isize`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
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
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Ord`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `isize`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        19,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `isize`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        23,
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
                                                                symbol_modifier_tokens: Some(
                                                                    Mut(
                                                                        MutRegionalToken {
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                9,
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `arr`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        10,
                                                                    ),
                                                                },
                                                            },
                                                            SynPatternExpr::Ident {
                                                                symbol_modifier_tokens: None,
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `low`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        17,
                                                                    ),
                                                                },
                                                            },
                                                            SynPatternExpr::Ident {
                                                                symbol_modifier_tokens: None,
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `high`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        21,
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
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `T`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                4,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Mut,
                                                                access_start: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                                    ident: `arr`,
                                                                    pattern_symbol_idx: 1,
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: None,
                                                                access_start: RegionalTokenIdx(
                                                                    18,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                                    ident: `low`,
                                                                    pattern_symbol_idx: 2,
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: None,
                                                                access_start: RegionalTokenIdx(
                                                                    22,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                            OrdinaryParenateParameter {
                                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                                    syn_pattern_expr_idx: 1,
                                                                },
                                                                ty_expr_idx: 4,
                                                            },
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                        (
                                                            OrdinaryParenateParameter {
                                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                                    syn_pattern_expr_idx: 2,
                                                                },
                                                                ty_expr_idx: 5,
                                                            },
                                                            ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        ),
                                                        (
                                                            OrdinaryParenateParameter {
                                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                                    syn_pattern_expr_idx: 3,
                                                                },
                                                                ty_expr_idx: 6,
                                                            },
                                                            ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [
                                                    SynPatternExprRoot {
                                                        kind: SynPatternExprRootKind::Parenate,
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    SynPatternExprRoot {
                                                        kind: SynPatternExprRootKind::Parenate,
                                                        syn_pattern_expr_idx: 2,
                                                    },
                                                    SynPatternExprRoot {
                                                        kind: SynPatternExprRootKind::Parenate,
                                                        syn_pattern_expr_idx: 3,
                                                    },
                                                ],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Traits,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ExplicitParameterType,
                                                        syn_expr_idx: 4,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ExplicitParameterType,
                                                        syn_expr_idx: 5,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ExplicitParameterType,
                                                        syn_expr_idx: 6,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::InheritedSynSymbol {
                                                ident: `low`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    2,
                                                ),
                                                inherited_syn_symbol_idx: 3,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `high`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                inherited_syn_symbol_idx: 4,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `high`,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 1,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    3,
                                                ),
                                                ropd: 2,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`quick_sort::partition`, `FunctionFn`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `low`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                                inherited_syn_symbol_idx: 3,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `high`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                inherited_syn_symbol_idx: 4,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `high`,
                                                },
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 4,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 5,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                12,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 6,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                14,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 7,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `low`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                                inherited_syn_symbol_idx: 3,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `p`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    23,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    25,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Binary {
                                                lopd: 12,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    24,
                                                ),
                                                ropd: 13,
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 9,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 10,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                20,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 11,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                22,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 14,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    26,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Fugitive(
                                                            FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    29,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `p`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    31,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    33,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Binary {
                                                lopd: 18,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                                ropd: 19,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `high`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    35,
                                                ),
                                                inherited_syn_symbol_idx: 4,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `high`,
                                                },
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 16,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    28,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 17,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                30,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 20,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                34,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 21,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    36,
                                                ),
                                            },
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    4..5,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `partition`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::partition`, `FunctionFn`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `quick_sort_aux`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `quick_sort_aux`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Fugitive(
                                                        FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 1,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 8,
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 15,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 22,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::IfElse {
                                                if_branch: SynIfBranch {
                                                    if_token: IfRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            1,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        3,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    ),
                                                    stmts: ArenaIdxRange(
                                                        1..4,
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
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `p`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
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
                                        inherited_syn_symbol_arena: Arena {
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
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                37,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 8,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 15,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 22,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 23,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    SynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::FunctionFn(
                FnSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: FnSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
                                disambiguator: 0,
                            },
                        },
                        template_parameter_obelisk_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterSyndicate {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            data: TemplateParameterSyndicateData::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                },
                                                traits: Some(
                                                    (
                                                        ColonRegionalToken(
                                                            RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        ),
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        parenate_parameter_obelisk_list: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [
                                    ParenateParameterSyndicate::Ordinary {
                                        syn_pattern_root: ParenateSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonRegionalToken(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                        ty: 4,
                                    },
                                    ParenateParameterSyndicate::Ordinary {
                                        syn_pattern_root: ParenateSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon: ColonRegionalToken(
                                            RegionalTokenIdx(
                                                18,
                                            ),
                                        ),
                                        ty: 5,
                                    },
                                    ParenateParameterSyndicate::Ordinary {
                                        syn_pattern_root: ParenateSynPatternExprRoot {
                                            syn_pattern_expr_idx: 3,
                                        },
                                        variables: ArenaIdxRange(
                                            4..5,
                                        ),
                                        colon: ColonRegionalToken(
                                            RegionalTokenIdx(
                                                22,
                                            ),
                                        ),
                                        ty: 6,
                                    },
                                ],
                                commas: [
                                    CommaRegionalToken(
                                        RegionalTokenIdx(
                                            16,
                                        ),
                                    ),
                                    CommaRegionalToken(
                                        RegionalTokenIdx(
                                            20,
                                        ),
                                    ),
                                ],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        24,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 7,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::cmp::Ord`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::BoxColonList {
                                            lbox_regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                            colon_regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                            items: [],
                                            rbox_regional_token_idx: RegionalTokenIdx(
                                                14,
                                            ),
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `T`,
                                            regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                            current_syn_symbol_idx: 1,
                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `T`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 2,
                                            argument_expr_idx: 3,
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 4,
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
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `Ord`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::Ord`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `isize`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::isize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `isize`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::isize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `isize`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        26,
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
                                                symbol_modifier_tokens: Some(
                                                    Mut(
                                                        MutRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                9,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `arr`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_tokens: None,
                                                ident_token: IdentRegionalToken {
                                                    ident: `low`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_tokens: None,
                                                ident_token: IdentRegionalToken {
                                                    ident: `high`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
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
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    5,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: Mut,
                                                access_start: RegionalTokenIdx(
                                                    11,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                    ident: `arr`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: RegionalTokenIdx(
                                                    18,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                    ident: `low`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: RegionalTokenIdx(
                                                    22,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 1,
                                                },
                                                ty_expr_idx: 4,
                                            },
                                            ArenaIdxRange(
                                                2..3,
                                            ),
                                        ),
                                        (
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 2,
                                                },
                                                ty_expr_idx: 5,
                                            },
                                            ArenaIdxRange(
                                                3..4,
                                            ),
                                        ),
                                        (
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 3,
                                                },
                                                ty_expr_idx: 6,
                                            },
                                            ArenaIdxRange(
                                                4..5,
                                            ),
                                        ),
                                    ],
                                },
                                syn_pattern_expr_roots: [
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 1,
                                    },
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 2,
                                    },
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 3,
                                    },
                                ],
                                syn_expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::Traits,
                                        syn_expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 4,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 5,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 6,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ReturnType,
                                        syn_expr_idx: 7,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                            },
                        },
                    },
                    body_with_syn_expr_region: Some(
                        (
                            63,
                            SynExprRegion {
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
                                                                    path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::cmp::Ord`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::BoxColonList {
                                                            lbox_regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                            colon_regional_token_idx: RegionalTokenIdx(
                                                                13,
                                                            ),
                                                            items: [],
                                                            rbox_regional_token_idx: RegionalTokenIdx(
                                                                14,
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                15,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `T`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::num::isize`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::num::isize`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
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
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Ord`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::cmp::Ord`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `isize`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        19,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `isize`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        23,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::num::isize`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `isize`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        26,
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
                                                                symbol_modifier_tokens: Some(
                                                                    Mut(
                                                                        MutRegionalToken {
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                9,
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `arr`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        10,
                                                                    ),
                                                                },
                                                            },
                                                            SynPatternExpr::Ident {
                                                                symbol_modifier_tokens: None,
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `low`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        17,
                                                                    ),
                                                                },
                                                            },
                                                            SynPatternExpr::Ident {
                                                                symbol_modifier_tokens: None,
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `high`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        21,
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
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `T`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                4,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Mut,
                                                                access_start: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                                    ident: `arr`,
                                                                    pattern_symbol_idx: 1,
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: None,
                                                                access_start: RegionalTokenIdx(
                                                                    18,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                                    ident: `low`,
                                                                    pattern_symbol_idx: 2,
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: None,
                                                                access_start: RegionalTokenIdx(
                                                                    22,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::ParenateRegularParameter {
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
                                                            OrdinaryParenateParameter {
                                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                                    syn_pattern_expr_idx: 1,
                                                                },
                                                                ty_expr_idx: 4,
                                                            },
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                        (
                                                            OrdinaryParenateParameter {
                                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                                    syn_pattern_expr_idx: 2,
                                                                },
                                                                ty_expr_idx: 5,
                                                            },
                                                            ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        ),
                                                        (
                                                            OrdinaryParenateParameter {
                                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                                    syn_pattern_expr_idx: 3,
                                                                },
                                                                ty_expr_idx: 6,
                                                            },
                                                            ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [
                                                    SynPatternExprRoot {
                                                        kind: SynPatternExprRootKind::Parenate,
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    SynPatternExprRoot {
                                                        kind: SynPatternExprRootKind::Parenate,
                                                        syn_pattern_expr_idx: 2,
                                                    },
                                                    SynPatternExprRoot {
                                                        kind: SynPatternExprRootKind::Parenate,
                                                        syn_pattern_expr_idx: 3,
                                                    },
                                                ],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Traits,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ExplicitParameterType,
                                                        syn_expr_idx: 4,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ExplicitParameterType,
                                                        syn_expr_idx: 5,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ExplicitParameterType,
                                                        syn_expr_idx: 6,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ReturnType,
                                                        syn_expr_idx: 7,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::InheritedSynSymbol {
                                                ident: `high`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                inherited_syn_symbol_idx: 4,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `high`,
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 1,
                                                opr: As,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                ropd: 2,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `low`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                inherited_syn_symbol_idx: 3,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `low`,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Binary {
                                                lopd: 4,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                ropd: 5,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `high`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                                inherited_syn_symbol_idx: 4,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `high`,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    20,
                                                ),
                                                LiteralData::Bool(
                                                    True,
                                                ),
                                            ),
                                            SynExprData::CurrentSynSymbol {
                                                ident: `store_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    22,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    24,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Binary {
                                                lopd: 9,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    23,
                                                ),
                                                ropd: 10,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    26,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `store_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    28,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 13,
                                                opr: As,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    29,
                                                ),
                                                ropd: 14,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    33,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `pivot`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    35,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 12,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    27,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 15,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    31,
                                                ),
                                            },
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 16,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    34,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 17,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    36,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 18,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                                ropd: 19,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `store_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    38,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    40,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Binary {
                                                lopd: 21,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    39,
                                                ),
                                                ropd: 22,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `last_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    41,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    43,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Binary {
                                                lopd: 24,
                                                opr: AssignClosed(
                                                    Sub,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    42,
                                                ),
                                                ropd: 25,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `last_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    45,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    47,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    49,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `last_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    51,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 30,
                                                opr: As,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    52,
                                                ),
                                                ropd: 31,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    56,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `pivot`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    58,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 29,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    50,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 32,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    54,
                                                ),
                                            },
                                            SynExprData::IndexOrCompositionWithList {
                                                owner: 33,
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    57,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 34,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    59,
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 27,
                                                opr: Comparison(
                                                    Geq,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    46,
                                                ),
                                                ropd: 28,
                                            },
                                            SynExprData::Binary {
                                                lopd: 35,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    55,
                                                ),
                                                ropd: 36,
                                            },
                                            SynExprData::Binary {
                                                lopd: 37,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    48,
                                                ),
                                                ropd: 38,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `last_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    61,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    63,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Binary {
                                                lopd: 40,
                                                opr: AssignClosed(
                                                    Sub,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    62,
                                                ),
                                                ropd: 41,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `store_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    65,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `last_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    67,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::Binary {
                                                lopd: 43,
                                                opr: Comparison(
                                                    Geq,
                                                ),
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    66,
                                                ),
                                                ropd: 44,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    72,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `store_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    76,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 47,
                                                opr: As,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    77,
                                                ),
                                                ropd: 48,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `last_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    80,
                                                ),
                                                current_syn_symbol_idx: 3,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 5,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 50,
                                                opr: As,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    81,
                                                ),
                                                ropd: 51,
                                            },
                                            SynExprData::MethodApplicationOrCall {
                                                self_argument: 46,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    73,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `swap`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        74,
                                                    ),
                                                },
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    75,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 49,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                79,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 52,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    83,
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `arr`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    84,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                    ident: `arr`,
                                                },
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `store_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    88,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 6,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 55,
                                                opr: As,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    89,
                                                ),
                                                ropd: 56,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `pivot`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    92,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 7,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Binary {
                                                lopd: 58,
                                                opr: As,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    93,
                                                ),
                                                ropd: 59,
                                            },
                                            SynExprData::MethodApplicationOrCall {
                                                self_argument: 54,
                                                dot_regional_token_idx: RegionalTokenIdx(
                                                    85,
                                                ),
                                                ident_token: IdentRegionalToken {
                                                    ident: `swap`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        86,
                                                    ),
                                                },
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    87,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 57,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                91,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 60,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    95,
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `store_index`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    96,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    10..16,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `usize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `usize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            30,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `usize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `usize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            78,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `usize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            82,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `usize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            90,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `usize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            94,
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
                                            SynStmtData::Eval {
                                                expr_idx: 23,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 42,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Break {
                                                break_token: BreakRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        69,
                                                    ),
                                                },
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 53,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 11,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::While {
                                                while_token: WhileRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                },
                                                condition: Ok(
                                                    20,
                                                ),
                                                eol_colon: Ok(
                                                    EolRegionalToken::Colon(
                                                        EolColonRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                37,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 26,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::While {
                                                while_token: WhileRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        44,
                                                    ),
                                                },
                                                condition: Ok(
                                                    39,
                                                ),
                                                eol_colon: Ok(
                                                    EolRegionalToken::Colon(
                                                        EolColonRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                60,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: ArenaIdxRange(
                                                    2..3,
                                                ),
                                            },
                                            SynStmtData::IfElse {
                                                if_branch: SynIfBranch {
                                                    if_token: IfRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            64,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        45,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                68,
                                                            ),
                                                        },
                                                    ),
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                                elif_branches: [],
                                                else_branch: Some(
                                                    SynElseBranch {
                                                        else_token: ElseRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                70,
                                                            ),
                                                        },
                                                        eol_colon_token: Ok(
                                                            EolColonRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    71,
                                                                ),
                                                            },
                                                        ),
                                                        stmts: ArenaIdxRange(
                                                            4..5,
                                                        ),
                                                    },
                                                ),
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        1,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 1,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 3,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 2,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 6,
                                            },
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 3,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    ),
                                                ),
                                                initial_value: 7,
                                            },
                                            SynStmtData::While {
                                                while_token: WhileRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                },
                                                condition: Ok(
                                                    8,
                                                ),
                                                eol_colon: Ok(
                                                    EolRegionalToken::Colon(
                                                        EolColonRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: ArenaIdxRange(
                                                    5..10,
                                                ),
                                            },
                                            SynStmtData::Eval {
                                                expr_idx: 61,
                                                eol_semicolon: Ok(
                                                    None,
                                                ),
                                            },
                                            SynStmtData::Eval {
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
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `pivot`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            2,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        Mut(
                                                            MutRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    8,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `store_index`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        Mut(
                                                            MutRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `last_index`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
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
                                        inherited_syn_symbol_arena: Arena {
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
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                97,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
                                                        ident: `pivot`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Mut,
                                                    access_start: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                97,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
                                                        ident: `store_index`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Mut,
                                                    access_start: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                97,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 1,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 2,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 3,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 6,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 7,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 11,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 23,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 26,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 42,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 53,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 61,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::EvalExpr,
                                            syn_expr_idx: 62,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 63,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    SynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::FunctionFn(
                FnSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: FnSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
                                disambiguator: 0,
                            },
                        },
                        template_parameter_obelisk_list: Ok(
                            None,
                        ),
                        parenate_parameter_obelisk_list: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            None,
                        ),
                        return_ty: Ok(
                            None,
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
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
                                    pattern_symbol_arena: Arena {
                                        data: [],
                                    },
                                    pattern_symbol_maps: [],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                syn_pattern_expr_roots: [],
                                syn_expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                            },
                        },
                    },
                    body_with_syn_expr_region: Some(
                        (
                            16,
                            SynExprRegion {
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
                                                                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
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
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SynSymbolRegion {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: False,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    10,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        4,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    12,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        65,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    14,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        2,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    17,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        31,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Prefix {
                                                opr: Minus,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                opd: 7,
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    19,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        0,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    21,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        99,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    23,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        2,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    25,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        83,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    27,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        782,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    29,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 4,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                11,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 5,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                13,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 6,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                15,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 8,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 9,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                20,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 10,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                22,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 11,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                24,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 12,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                26,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 13,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                28,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 14,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    30,
                                                ),
                                            },
                                            SynExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
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
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        1,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                        variables: ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                        colon_token: Ok(
                                                            Some(
                                                                ColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        4,
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            8,
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
                                                    symbol_modifier_tokens: Some(
                                                        Mut(
                                                            MutRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    2,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `v`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
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
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Mut,
                                                    access_start: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                31,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                                LetPattern {
                                                    pattern: LetSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtType,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 15,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 16,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    SynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Fugitive(
            FugitiveSynNodeDefn::FunctionFn(
                FnSynNodeDefn {
                    syn_node_path: FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: FnSynNodeDecl {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
                                disambiguator: 0,
                            },
                        },
                        template_parameter_obelisk_list: Ok(
                            None,
                        ),
                        parenate_parameter_obelisk_list: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            None,
                        ),
                        return_ty: Ok(
                            None,
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
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
                                    pattern_symbol_arena: Arena {
                                        data: [],
                                    },
                                    pattern_symbol_maps: [],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                syn_pattern_expr_roots: [],
                                syn_expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                            },
                        },
                    },
                    body_with_syn_expr_region: Some(
                        (
                            8,
                            SynExprRegion {
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
                                                                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
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
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SynSymbolRegion {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: False,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                                LiteralData::String(
                                                    StringLiteralData {
                                                        data: "beach",
                                                    },
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                                LiteralData::String(
                                                    StringLiteralData {
                                                        data: "hotel",
                                                    },
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    10,
                                                ),
                                                LiteralData::String(
                                                    StringLiteralData {
                                                        data: "airplane",
                                                    },
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    12,
                                                ),
                                                LiteralData::String(
                                                    StringLiteralData {
                                                        data: "car",
                                                    },
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    14,
                                                ),
                                                LiteralData::String(
                                                    StringLiteralData {
                                                        data: "house",
                                                    },
                                                ),
                                            ),
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    16,
                                                ),
                                                LiteralData::String(
                                                    StringLiteralData {
                                                        data: "art",
                                                    },
                                                ),
                                            ),
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 1,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 2,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                9,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 3,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                11,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 4,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                13,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 5,
                                                        comma_regional_token_idx: Some(
                                                            RegionalTokenIdx(
                                                                15,
                                                            ),
                                                        ),
                                                    },
                                                    SynCommaListItem {
                                                        syn_expr_idx: 6,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                            },
                                            SynExprData::Block {
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
                                            SynStmtData::Let {
                                                let_token: LetRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        1,
                                                    ),
                                                },
                                                let_variables_pattern: Ok(
                                                    LetPatternSynSyndicate {
                                                        syn_pattern_expr_root: LetSynPatternExprRoot {
                                                            syn_pattern_expr_idx: 1,
                                                        },
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
                                                    EqRegionalToken(
                                                        RegionalTokenIdx(
                                                            4,
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
                                                    symbol_modifier_tokens: Some(
                                                        Mut(
                                                            MutRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    2,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `strs`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
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
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Mut,
                                                    access_start: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    access_end: Some(
                                                        RegionalTokenIdxRangeEnd(
                                                            RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        ),
                                                    ),
                                                    data: CurrentSynSymbolData::LetVariable {
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
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Let,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::LetStmtInitialValue,
                                            syn_expr_idx: 7,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::BlockExpr,
                                            syn_expr_idx: 8,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]