```rust
SynNodeDeclSheet {
    decls: [
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypePath(`malamute::Class`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Type(
                    TypeSynNodeDecl::Enum(
                        EnumSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`malamute::Class`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameter_decl_list: Ok(
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
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtRegionalToken(
                                            RegionalTokenIdx(
                                                6,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`malamute::Class`, `Enum`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
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
                                    pattern_expr_region: SynPatternRegion {
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Const,
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
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [],
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
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
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Type(
                    TypeSynNodeDecl::Enum(
                        EnumSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameter_decl_list: Ok(
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
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            TemplateSynParameterData {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: TemplateParameterSyndicateVariant::Compterm {
                                                    termic_token: TermicRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                    colon_token: ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    ),
                                                    ty_expr: 0,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::CurrentSynSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [
                                                                Phantom(
                                                                    PoundRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    PhanRegionalToken {
                                                                        token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [
                                                                Phantom(
                                                                    PoundRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            9,
                                                                        ),
                                                                    ),
                                                                    PhanRegionalToken {
                                                                        token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    12,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ConstantImplicitParameterType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
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
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Type(
                    TypeSynNodeDecl::Enum(
                        EnumSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameter_decl_list: Ok(
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
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            TemplateSynParameterData {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: TemplateParameterSyndicateVariant::Compterm {
                                                    termic_token: TermicRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                    colon_token: ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    ),
                                                    ty_expr: 0,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::CurrentSynSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [
                                                                Phantom(
                                                                    PoundRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    PhanRegionalToken {
                                                                        token_idx: RegionalTokenIdx(
                                                                            6,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [
                                                                Phantom(
                                                                    PoundRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            9,
                                                                        ),
                                                                    ),
                                                                    PhanRegionalToken {
                                                                        token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    12,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ConstantImplicitParameterType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
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
                    FormSynNodePath(`malamute::narrow_down`, `Ritchie(
                        Gn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: FormSynNodePath(`malamute::narrow_down`, `Ritchie(
                                Gn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Gn,
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
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            TemplateSynParameterData {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: TemplateParameterSyndicateVariant::Compterm {
                                                    termic_token: TermicRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    colon_token: ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    ),
                                                    ty_expr: 0,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtRegionalToken(
                                            RegionalTokenIdx(
                                                13,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            parenate_parameter_obelisk_list: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            14,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Variadic {
                                            dot_dot_dot_token: DotDotDotRegionalToken(
                                                RegionalTokenIdx(
                                                    15,
                                                ),
                                            ),
                                            variadic_variant: SynVariadicParameterVariant::Vec {
                                                lbox_token: LboxRegionalToken(
                                                    RegionalTokenIdx(
                                                        16,
                                                    ),
                                                ),
                                                rbox_token: RboxRegionalToken(
                                                    RegionalTokenIdx(
                                                        17,
                                                    ),
                                                ),
                                            },
                                            symbol_modifier_keyword_group: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `f`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                            },
                                            variable: 2,
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    19,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        ParenateParameterSyndicate::Keyed {
                                            syn_pattern_root: ParenateParameterSynPatternRoot {
                                                syn_pattern_idx: 0,
                                            },
                                            symbol_modifier_keyword_group: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `skip`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    22,
                                                ),
                                            },
                                            variable: 3,
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    23,
                                                ),
                                            ),
                                            ty: 2,
                                            eq_token: EqRegionalToken(
                                                RegionalTokenIdx(
                                                    25,
                                                ),
                                            ),
                                            default: Right(
                                                3,
                                            ),
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                21,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                27,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            28,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            29,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 8,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            33,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                FormSynNodePath(`malamute::narrow_down`, `Ritchie(
                                                    Gn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::CurrentSynSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
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
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    26,
                                                ),
                                                LiteralTokenData::Integer(
                                                    UnspecifiedRegular(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    31,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 4,
                                                argument_expr_idx: 5,
                                            },
                                            SynExprData::CurrentSynSymbol {
                                                ident: `label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                                current_variable_idx: 1,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 6,
                                                argument_expr_idx: 7,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `OneVsAllResult`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            30,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `skip`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
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
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `skip`,
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Const,
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
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [
                                                                Runtime(
                                                                    PoundRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    ),
                                                                    PolyRegionalToken {
                                                                        token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::VariadicParenateParameter {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `f`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `skip`,
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
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                VariadicParenateParameter {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 34,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 2,
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
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ConstantImplicitParameterType,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ParenateParameterDefaultValue {
                                                ty_syn_expr_idx: 2,
                                            },
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 8,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [
                                        (
                                            0,
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
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockSynNodePathData {
                                        path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::ImplBlock(
                ImplBlockSynNodeDecl::TraitForType(
                    TraitForTypeImplBlockSynNodeDecl {
                        syn_node_path: TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                        impl_regional_token: ImplRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                        },
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            2,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterSyndicateVariant::Compterm {
                                                termic_token: TermicRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                                colon_token: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                ty_expr: 0,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                6,
                                            ),
                                        ),
                                    ],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        trai_expr: TraitSyndicate {
                            syn_expr_idx: 1,
                        },
                        for_token: ConnectionForRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                15,
                            ),
                        },
                        self_ty_decl: PathLeadingExpr(
                            SelfTypeSyndicate {
                                expr: 6,
                            },
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
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::ImplBlock(
                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::ImplBlock(
                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePathData {
                                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::CurrentSynSymbol {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                            current_variable_idx: 0,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::default::Default`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                17,
                                            ),
                                            current_variable_idx: 0,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
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
                                        SynExprData::CurrentSynSymbol {
                                            ident: `label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                18,
                                            ),
                                            current_variable_idx: 1,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 4,
                                            argument_expr_idx: 5,
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `Default`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::default::Default`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `OneVsAll`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: SynPatternRegion {
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
                                variable_region: VariableRegionData {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [
                                                            Phantom(
                                                                PoundRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                ),
                                                                PhanRegionalToken {
                                                                    token_idx: RegionalTokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                            CurrentVariableEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    13,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [
                                                            Phantom(
                                                                PoundRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        7,
                                                                    ),
                                                                ),
                                                                PhanRegionalToken {
                                                                    token_idx: RegionalTokenIdx(
                                                                        8,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Constant {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                        ty_expr_idx: 0,
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                        syn_expr_idx: 0,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::PrimalTrait,
                                        syn_expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::SelfType,
                                        syn_expr_idx: 6,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                `<malamute::OneVsAll as core::default::Default(0)>::default`,
                                                TraitItemKind::AssocRitchie(
                                                    RitchieItemKind::Fn,
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TraitForTypeItem(
                    TraitForTypeItemSynNodeDecl::AssocRitchie(
                        TraitForTypeAssocRitchieSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                        `<malamute::OneVsAll as core::default::Default(0)>::default`,
                                                        TraitItemKind::AssocRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 0,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::default::Default`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
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
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Default`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::default::Default`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternRegion {
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        4,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        },
                                                                        trai_syn_expr_idxs: [],
                                                                    },
                                                                },
                                                            },
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        7,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        8,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 6,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                            `<malamute::OneVsAll as core::default::Default(0)>::default`,
                                                                            TraitItemKind::AssocRitchie(
                                                                                RitchieItemKind::Fn,
                                                                            ),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::SelfType(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
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
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockSynNodePathData {
                                        path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::ImplBlock(
                ImplBlockSynNodeDecl::TraitForType(
                    TraitForTypeImplBlockSynNodeDecl {
                        syn_node_path: TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                        impl_regional_token: ImplRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                        },
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            2,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterSyndicateVariant::Compterm {
                                                termic_token: TermicRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                                colon_token: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty_expr: 0,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                4,
                                            ),
                                        ),
                                    ],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            11,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        trai_expr: TraitSyndicate {
                            syn_expr_idx: 7,
                        },
                        for_token: ConnectionForRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                20,
                            ),
                        },
                        self_ty_decl: PathLeadingExpr(
                            SelfTypeSyndicate {
                                expr: 10,
                            },
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::ImplBlock(
                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::ImplBlock(
                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePathData {
                                                                path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::CurrentSynSymbol {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                10,
                                            ),
                                            current_variable_idx: 0,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::ops::Unveil`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 1,
                                            argument_expr_idx: 2,
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                18,
                                            ),
                                            current_variable_idx: 0,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 3,
                                            argument_expr_idx: 4,
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                19,
                                            ),
                                            current_variable_idx: 1,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 5,
                                            argument_expr_idx: 6,
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::Class`, `Enum`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                22,
                                            ),
                                            current_variable_idx: 0,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 8,
                                            argument_expr_idx: 9,
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `core`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::Module(
                                                `core`,
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Subitem {
                                            parent: 0,
                                            colon_colon_token: ColonColonRegionalToken(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentRegionalToken {
                                                    ident: `ops`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::Module(
                                                    `core::ops`,
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Subitem {
                                            parent: 1,
                                            colon_colon_token: ColonColonRegionalToken(
                                                RegionalTokenIdx(
                                                    15,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentRegionalToken {
                                                    ident: `Unveil`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::ops::Unveil`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `OneVsAll`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `Class`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::Class`, `Enum`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: SynPatternRegion {
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
                                variable_region: VariableRegionData {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    4,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                            CurrentVariableEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    11,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [
                                                            Runtime(
                                                                PoundRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                ),
                                                                PolyRegionalToken {
                                                                    token_idx: RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Constant {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
                                                            ),
                                                        },
                                                        ty_expr_idx: 0,
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                        syn_expr_idx: 0,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::PrimalTrait,
                                        syn_expr_idx: 7,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::SelfType,
                                        syn_expr_idx: 10,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                `<malamute::Class as core::ops::Unveil(0)>::Output`,
                                                TraitItemKind::AssocType,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TraitForTypeItem(
                    TraitForTypeItemSynNodeDecl::AssocType(
                        TraitForTypeAssocTypeSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                        `<malamute::Class as core::ops::Unveil(0)>::Output`,
                                                        TraitItemKind::AssocType,
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                            ),
                            ty_term_expr_idx: 0,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::Class`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `core`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        12,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        17,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Class`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        21,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::Class`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternRegion {
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                        trai_syn_expr_idxs: [],
                                                                    },
                                                                },
                                                            },
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Runtime(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                PolyRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        6,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                8,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 10,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                            `<malamute::Class as core::ops::Unveil(0)>::Output`,
                                                                            TraitItemKind::AssocType,
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::Unit {
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::AssocTypeTerm,
                                            syn_expr_idx: 0,
                                        },
                                    ],
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
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                                                TraitItemKind::AssocRitchie(
                                                    RitchieItemKind::Fn,
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TraitForTypeItem(
                    TraitForTypeItemSynNodeDecl::AssocRitchie(
                        TraitForTypeAssocRitchieSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                        `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                                                        TraitItemKind::AssocRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternRoot {
                                                syn_pattern_idx: 0,
                                            },
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            11,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 8,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::Class`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `core`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        12,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        17,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Class`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        21,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::Class`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternRegion {
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                        trai_syn_expr_idxs: [],
                                                                    },
                                                                },
                                                            },
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Runtime(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                PolyRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        6,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                8,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 10,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                            `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                                                                            TraitItemKind::AssocRitchie(
                                                                                RitchieItemKind::Fn,
                                                                            ),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                inherited_syn_symbol_idx: 0,
                                                inherited_syn_symbol_kind: InheritedVariableKind::Template(
                                                    InheritedTemplateVariable::Type {
                                                        ident: `Label`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedVariableKind::Template(
                                                    InheritedTemplateVariable::Constant {
                                                        ident: `label`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::SelfType(
                                                RegionalTokenIdx(
                                                    17,
                                                ),
                                            ),
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 7,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `OneVsAll`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `core`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `core`,
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        13,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ops`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        `core::ops`,
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 2,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        15,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ControlFlow`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `one_vs_all`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
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
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `one_vs_all`,
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `one_vs_all`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
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
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 8,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [
                                        (
                                            0,
                                            0,
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
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockSynNodePathData {
                                        path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::ImplBlock(
                ImplBlockSynNodeDecl::TraitForType(
                    TraitForTypeImplBlockSynNodeDecl {
                        syn_node_path: TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                        impl_regional_token: ImplRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                        },
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            2,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterSyndicateVariant::Compterm {
                                                termic_token: TermicRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                                colon_token: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                ty_expr: 0,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                6,
                                            ),
                                        ),
                                    ],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        trai_expr: TraitSyndicate {
                            syn_expr_idx: 7,
                        },
                        for_token: ConnectionForRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                22,
                            ),
                        },
                        self_ty_decl: PathLeadingExpr(
                            SelfTypeSyndicate {
                                expr: 12,
                            },
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
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::ImplBlock(
                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::ImplBlock(
                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePathData {
                                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExprData::CurrentSynSymbol {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                12,
                                            ),
                                            current_variable_idx: 0,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::ops::Unveil`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 1,
                                            argument_expr_idx: 2,
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                20,
                                            ),
                                            current_variable_idx: 0,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 3,
                                            argument_expr_idx: 4,
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                21,
                                            ),
                                            current_variable_idx: 1,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 5,
                                            argument_expr_idx: 6,
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                24,
                                            ),
                                            current_variable_idx: 0,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 8,
                                            argument_expr_idx: 9,
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                25,
                                            ),
                                            current_variable_idx: 1,
                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 10,
                                            argument_expr_idx: 11,
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `core`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::Module(
                                                `core`,
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Subitem {
                                            parent: 0,
                                            colon_colon_token: ColonColonRegionalToken(
                                                RegionalTokenIdx(
                                                    15,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentRegionalToken {
                                                    ident: `ops`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::Module(
                                                    `core::ops`,
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Subitem {
                                            parent: 1,
                                            colon_colon_token: ColonColonRegionalToken(
                                                RegionalTokenIdx(
                                                    17,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentRegionalToken {
                                                    ident: `Unveil`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        18,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::ops::Unveil`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `OneVsAllResult`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `OneVsAll`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: SynPatternRegion {
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
                                variable_region: VariableRegionData {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [
                                                            Phantom(
                                                                PoundRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                ),
                                                                PhanRegionalToken {
                                                                    token_idx: RegionalTokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                            CurrentVariableEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    13,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [
                                                            Phantom(
                                                                PoundRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        7,
                                                                    ),
                                                                ),
                                                                PhanRegionalToken {
                                                                    token_idx: RegionalTokenIdx(
                                                                        8,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Constant {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                        ty_expr_idx: 0,
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                        syn_expr_idx: 0,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::PrimalTrait,
                                        syn_expr_idx: 7,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::SelfType,
                                        syn_expr_idx: 12,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                `<malamute::OneVsAll as core::ops::Unveil(0)>::Output`,
                                                TraitItemKind::AssocType,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TraitForTypeItem(
                    TraitForTypeItemSynNodeDecl::AssocType(
                        TraitForTypeAssocTypeSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::Output`,
                                                        TraitItemKind::AssocType,
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                            ),
                            ty_term_expr_idx: 0,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                20,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                24,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                25,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 10,
                                                            argument_expr_idx: 11,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `core`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    17,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        18,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAllResult`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        19,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        23,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternRegion {
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        4,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        },
                                                                        trai_syn_expr_idxs: [],
                                                                    },
                                                                },
                                                            },
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        7,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        8,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 12,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                            `<malamute::OneVsAll as core::ops::Unveil(0)>::Output`,
                                                                            TraitItemKind::AssocType,
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::Unit {
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::AssocTypeTerm,
                                            syn_expr_idx: 0,
                                        },
                                    ],
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
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TraitForTypeItem(
                                    TraitForTypeItemSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                                TraitItemKind::AssocRitchie(
                                                    RitchieItemKind::Fn,
                                                ),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TraitForTypeItem(
                    TraitForTypeItemSynNodeDecl::AssocRitchie(
                        TraitForTypeAssocRitchieSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                                        TraitItemKind::AssocRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternRoot {
                                                syn_pattern_idx: 0,
                                            },
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            10,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            11,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 8,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                20,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                24,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                25,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 10,
                                                            argument_expr_idx: 11,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `core`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    17,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        18,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAllResult`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        19,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        23,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternRegion {
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        4,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        },
                                                                        trai_syn_expr_idxs: [],
                                                                    },
                                                                },
                                                            },
                                                            CurrentVariableEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        7,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        8,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 12,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                            `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                                                            TraitItemKind::AssocRitchie(
                                                                                RitchieItemKind::Fn,
                                                                            ),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                inherited_syn_symbol_idx: 0,
                                                inherited_syn_symbol_kind: InheritedVariableKind::Template(
                                                    InheritedTemplateVariable::Type {
                                                        ident: `Label`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedVariableKind::Template(
                                                    InheritedTemplateVariable::Constant {
                                                        ident: `label`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::SelfType(
                                                RegionalTokenIdx(
                                                    17,
                                                ),
                                            ),
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 7,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `OneVsAllResult`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `core`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `core`,
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        13,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ops`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        `core::ops`,
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 2,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        15,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ControlFlow`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `one_vs_all_result`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
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
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `one_vs_all_result`,
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedVariable {
                                                    modifier: Const,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `one_vs_all_result`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
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
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 8,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [
                                        (
                                            0,
                                            0,
                                        ),
                                    ],
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