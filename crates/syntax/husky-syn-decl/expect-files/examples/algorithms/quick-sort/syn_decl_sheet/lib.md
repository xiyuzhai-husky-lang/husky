Ok(
    SynDeclSheet {
        [salsa id]: 24,
        decls: [
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::FunctionFn(
                            FunctionFnFugitiveSynDecl {
                                path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
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
                                return_ty: None,
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
                                                SynExprData::CurrentSymbol {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    SynCurrentSymbol {
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                                    SynCurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::ParenateRegularParameter {
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::FunctionFn(
                            FunctionFnFugitiveSynDecl {
                                path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
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
                                return_ty: None,
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
                                                SynExprData::CurrentSymbol {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    SynCurrentSymbol {
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                                    SynCurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::ParenateRegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    SynCurrentSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::ParenateRegularParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    SynCurrentSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::ParenateRegularParameter {
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::partition`, `FunctionFn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::FunctionFn(
                            FunctionFnFugitiveSynDecl {
                                path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
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
                                return_ty: Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 7,
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
                                                SynExprData::CurrentSymbol {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    SynCurrentSymbol {
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                                    SynCurrentSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::ParenateRegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    SynCurrentSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::ParenateRegularParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    SynCurrentSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                        access_end: None,
                                                        variant: SynCurrentSymbolVariant::ParenateRegularParameter {
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::FunctionFn(
                            FunctionFnFugitiveSynDecl {
                                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
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
                                        syn_pattern_expr_roots: [],
                                        syn_expr_roots: [],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::FunctionFn(
                            FunctionFnFugitiveSynDecl {
                                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
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
                                        syn_pattern_expr_roots: [],
                                        syn_expr_roots: [],
                                        has_self_lifetime: false,
                                        has_self_place: false,
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