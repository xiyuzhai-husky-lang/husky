Ok(
    [
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Add`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Add`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 33,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            15,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        16,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            17,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Add`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    17,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                16,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::AddAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::AddAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 35,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            44,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        45,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            46,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::AddAssign`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    46,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                45,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitAnd`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitAnd`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 37,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            73,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        74,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            75,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitAnd`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    75,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                74,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitAndAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitAndAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 39,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            102,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        103,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            104,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitAndAssign`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    104,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                103,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitOr`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitOr`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 41,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            127,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        128,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            129,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitOr`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    129,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                128,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitOrAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitOrAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 43,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            156,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        157,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            158,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitOrAssign`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    158,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                157,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitXor`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitXor`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 45,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            181,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        182,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            183,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitXor`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    183,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                182,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitXorAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitXorAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 47,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            210,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        211,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            212,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitXorAssign`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    212,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                211,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Div`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Div`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 49,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            235,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        236,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            237,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Div`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    237,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                236,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::DivAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::DivAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 51,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            264,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        265,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            266,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::DivAssign`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    266,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                265,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::IntIndex`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::IntIndex`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 52,
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::IntIndex`),
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
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Mul`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Mul`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 54,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            296,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        297,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            298,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Mul`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    298,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                297,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::MulAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::MulAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 56,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            325,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        326,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            327,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::MulAssign`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    327,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                326,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Neg`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Neg`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 58,
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Neg`),
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
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Not`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Not`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 60,
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Not`),
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
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Sub`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Sub`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 62,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            398,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        399,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            400,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Sub`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    400,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                399,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Unveil`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Unveil`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 63,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LaOrLtToken(
                                        TokenIdx(
                                            420,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterObelisk {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `T`,
                                                    token_idx: TokenIdx(
                                                        421,
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
                                    rangle: RaOrGtToken(
                                        TokenIdx(
                                            422,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Unveil`),
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
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    422,
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
                                                                421,
                                                            ),
                                                        },
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
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Type(
                TypeSynNodeDefn::Enum(
                    EnumTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: EnumTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 64,
                            template_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LaOrLtToken(
                                            TokenIdx(
                                                449,
                                            ),
                                        ),
                                        template_parameters: [
                                            TemplateParameterObelisk {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `R`,
                                                        token_idx: TokenIdx(
                                                            450,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            TemplateParameterObelisk {
                                                annotated_variance_token: None,
                                                symbol: 2,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `C`,
                                                        token_idx: TokenIdx(
                                                            452,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    451,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtToken(
                                            TokenIdx(
                                                453,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`core::ops::ControlFlow`, `Enum`),
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
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        451,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `R`,
                                                                token_idx: TokenIdx(
                                                                    450,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        453,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `C`,
                                                                token_idx: TokenIdx(
                                                                    452,
                                                                ),
                                                            },
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
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)