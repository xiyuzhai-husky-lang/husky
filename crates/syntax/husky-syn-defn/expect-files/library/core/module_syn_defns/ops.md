Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::Add`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::Add`),
                        ast_idx: 33,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::AddAssign`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::AddAssign`),
                        ast_idx: 35,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::BitAnd`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::BitAnd`),
                        ast_idx: 37,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::BitAndAssign`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::BitAndAssign`),
                        ast_idx: 39,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::BitOr`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::BitOr`),
                        ast_idx: 41,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::BitOrAssign`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::BitOrAssign`),
                        ast_idx: 43,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::BitXor`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::BitXor`),
                        ast_idx: 45,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::BitXorAssign`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::BitXorAssign`),
                        ast_idx: 47,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::Div`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::Div`),
                        ast_idx: 49,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::DivAssign`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::DivAssign`),
                        ast_idx: 51,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::IntIndex`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::IntIndex`),
                        ast_idx: 52,
                        template_parameters: [],
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::Mul`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::Mul`),
                        ast_idx: 54,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::MulAssign`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::MulAssign`),
                        ast_idx: 56,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::Neg`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::Neg`),
                        ast_idx: 58,
                        template_parameters: [],
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::Not`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::Not`),
                        ast_idx: 60,
                        template_parameters: [],
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::Sub`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::Sub`),
                        ast_idx: 62,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Trait(
                TraitSynDefn {
                    path: TraitPath(`core::ops::Unveil`),
                    decl: TraitSynDecl {
                        path: TraitPath(`core::ops::Unveil`),
                        ast_idx: 63,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeSynDefn::Enum(
                    EnumTypeSynDefn {
                        path: TypePath(`core::ops::ControlFlow`, `Enum`),
                        decl: EnumTypeSynDecl {
                            path: TypePath(`core::ops::ControlFlow`, `Enum`),
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