```rust
SynNodeDeclSheet {
    decls: [
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameter_obelisk_list: Ok(
                                Some(
                                    SynTemplateParameterSyndicateList {
                                        langle: LaOrLtRegionalToken(
                                            RegionalTokenIdx(
                                                4,
                                            ),
                                        ),
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
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
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
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            18,
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
                                            19,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
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
                                            SynExprData::CurrentVariable {
                                                ident: `T`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
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
                                        pattern_contracts: [
                                            Contract::BorrowMut,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `arr`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                RefMut,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Compterm,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `arr`,
                                                        pattern_variable_idx: 0,
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
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::TraitInConstraint,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 3,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameter_obelisk_list: Ok(
                                Some(
                                    SynTemplateParameterSyndicateList {
                                        langle: LaOrLtRegionalToken(
                                            RegionalTokenIdx(
                                                3,
                                            ),
                                        ),
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
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
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
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
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
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 2,
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
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                17,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                21,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            25,
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
                                            26,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
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
                                            SynExprData::CurrentVariable {
                                                ident: `T`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
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
                                        pattern_contracts: [
                                            Contract::BorrowMut,
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                                PatternVariable::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
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
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                RefMut,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Compterm,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `arr`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `low`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `high`,
                                                        pattern_variable_idx: 2,
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
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 2,
                                                    },
                                                    ty: 5,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 2,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::TraitInConstraint,
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
                                    pattern_to_current_variable_map: [
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`quick_sort::partition`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`quick_sort::partition`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameter_obelisk_list: Ok(
                                Some(
                                    SynTemplateParameterSyndicateList {
                                        langle: LaOrLtRegionalToken(
                                            RegionalTokenIdx(
                                                3,
                                            ),
                                        ),
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
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
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
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
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
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 2,
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
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                17,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                21,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            25,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            26,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 6,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::partition`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
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
                                            SynExprData::CurrentVariable {
                                                ident: `T`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
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
                                        pattern_contracts: [
                                            Contract::BorrowMut,
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                                PatternVariable::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
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
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                RefMut,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Compterm,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `arr`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `low`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `high`,
                                                        pattern_variable_idx: 2,
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
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 2,
                                                    },
                                                    ty: 5,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 2,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::TraitInConstraint,
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
                                    pattern_to_current_variable_map: [
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
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