```rust
SynDeclSheet {
    decls: [
        (
            ItemPath::MajorItem(
                MajorItemPath::Fugitive(
                    FugitivePath(`quick_sort::quick_sort`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
            SynDecl::MajorItem(
                MajorItemSynDecl::Fugitive(
                    FugitiveSynDecl::Fn(
                        MajorFnSynDecl {
                            path: FugitivePath(`quick_sort::quick_sort`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: [
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterSyndicateVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `T`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                        traits: Some(
                                            TraitsSyndicate {
                                                colon_regional_token: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        6,
                                                    ),
                                                ),
                                                traits_syn_expr_idx: 0,
                                                trait_syn_expr_idxs: [
                                                    0,
                                                ],
                                            },
                                        ),
                                    },
                                },
                            ],
                            parenate_parameters: [
                                ParenateParameterSyndicate::Simple {
                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                        syn_pattern_expr_idx: 0,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                    ty: 3,
                                },
                            ],
                            return_ty: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
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
                                                    14,
                                                ),
                                                colon_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `T`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                current_syn_symbol_idx: 0,
                                                current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
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
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        RefMut(
                                                            RefRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                            MutRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `arr`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                BorrowMut,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
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
                                                RefMut,
                                            ],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbolEntry {
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
                                                CurrentSynSymbolEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
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
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 0,
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
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::Trait,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 3,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            0,
                                            1,
                                        ),
                                    ],
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
                    FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
            SynDecl::MajorItem(
                MajorItemSynDecl::Fugitive(
                    FugitiveSynDecl::Fn(
                        MajorFnSynDecl {
                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: [
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterSyndicateVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `T`,
                                            regional_token_idx: RegionalTokenIdx(
                                                4,
                                            ),
                                        },
                                        traits: Some(
                                            TraitsSyndicate {
                                                colon_regional_token: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                traits_syn_expr_idx: 0,
                                                trait_syn_expr_idxs: [
                                                    0,
                                                ],
                                            },
                                        ),
                                    },
                                },
                            ],
                            parenate_parameters: [
                                ParenateParameterSyndicate::Simple {
                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                        syn_pattern_expr_idx: 0,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                    ty: 3,
                                },
                                ParenateParameterSyndicate::Simple {
                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            19,
                                        ),
                                    ),
                                    ty: 4,
                                },
                                ParenateParameterSyndicate::Simple {
                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                        syn_pattern_expr_idx: 2,
                                    },
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            23,
                                        ),
                                    ),
                                    ty: 5,
                                },
                            ],
                            return_ty: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
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
                                                current_syn_symbol_idx: 0,
                                                current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
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
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
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
                                                            20,
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
                                                            24,
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        RefMut(
                                                            RefRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            },
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `low`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `high`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                BorrowMut,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    0,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                                SynPatternSymbol::Atom(
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
                                                RefMut,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbolEntry {
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
                                                CurrentSynSymbolEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSynSymbolEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `low`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbolEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
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
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 0,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 2,
                                                    },
                                                    ty: 5,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 2,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::Trait,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 5,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            0,
                                            1,
                                        ),
                                        (
                                            1,
                                            2,
                                        ),
                                        (
                                            2,
                                            3,
                                        ),
                                    ],
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
                    FugitivePath(`quick_sort::partition`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
            SynDecl::MajorItem(
                MajorItemSynDecl::Fugitive(
                    FugitiveSynDecl::Fn(
                        MajorFnSynDecl {
                            path: FugitivePath(`quick_sort::partition`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: [
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterSyndicateVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `T`,
                                            regional_token_idx: RegionalTokenIdx(
                                                4,
                                            ),
                                        },
                                        traits: Some(
                                            TraitsSyndicate {
                                                colon_regional_token: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                traits_syn_expr_idx: 0,
                                                trait_syn_expr_idxs: [
                                                    0,
                                                ],
                                            },
                                        ),
                                    },
                                },
                            ],
                            parenate_parameters: [
                                ParenateParameterSyndicate::Simple {
                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                        syn_pattern_expr_idx: 0,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                    ty: 3,
                                },
                                ParenateParameterSyndicate::Simple {
                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            19,
                                        ),
                                    ),
                                    ty: 4,
                                },
                                ParenateParameterSyndicate::Simple {
                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                        syn_pattern_expr_idx: 2,
                                    },
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            23,
                                        ),
                                    ),
                                    ty: 5,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 6,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(`quick_sort::partition`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
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
                                                current_syn_symbol_idx: 0,
                                                current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
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
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
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
                                                            20,
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
                                                            24,
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
                                                            27,
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        RefMut(
                                                            RefRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            },
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `low`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `high`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                BorrowMut,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    0,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                                SynPatternSymbol::Atom(
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
                                                RefMut,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbolEntry {
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
                                                CurrentSynSymbolEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `arr`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSynSymbolEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `low`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbolEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
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
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 0,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 2,
                                                    },
                                                    ty: 5,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 2,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::Trait,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 3,
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
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 6,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            0,
                                            1,
                                        ),
                                        (
                                            1,
                                            2,
                                        ),
                                        (
                                            2,
                                            3,
                                        ),
                                    ],
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
                    FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
            SynDecl::MajorItem(
                MajorItemSynDecl::Fugitive(
                    FugitiveSynDecl::Fn(
                        MajorFnSynDecl {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: [],
                            parenate_parameters: [],
                            return_ty: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
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
                                    symbol_region: VariableRegionData {
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
                                    syn_pattern_to_current_syn_symbol_map: [],
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
                    FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
            SynDecl::MajorItem(
                MajorItemSynDecl::Fugitive(
                    FugitiveSynDecl::Fn(
                        MajorFnSynDecl {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: [],
                            parenate_parameters: [],
                            return_ty: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
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
                                    symbol_region: VariableRegionData {
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
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
    ],
}
```