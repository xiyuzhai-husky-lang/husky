Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeDefn::Extern(
                    ExternTypeSynDefn {
                        path: TypePath(`core::mem::Ref`, `Extern`),
                        decl: ExternTypeSynDecl {
                            path: TypePath(`core::mem::Ref`, `Extern`),
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    8,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Lifetime {
                                        label_token: LifetimeToken {
                                            label: `'a`,
                                            token_idx: TokenIdx(
                                                9,
                                            ),
                                        },
                                    },
                                },
                                TemplateParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    11,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                12,
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
                                                        path: TypePath(`core::mem::Ref`, `Extern`),
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
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: Some(
                                                            VarianceToken::Covariant(
                                                                CovariantToken {
                                                                    token_idx: TokenIdx(
                                                                        8,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Lifetime {
                                                            label_token: LifetimeToken {
                                                                label: `'a`,
                                                                token_idx: TokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: Some(
                                                            VarianceToken::Covariant(
                                                                CovariantToken {
                                                                    token_idx: TokenIdx(
                                                                        11,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    12,
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
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeDefn::Extern(
                    ExternTypeSynDefn {
                        path: TypePath(`core::mem::RefMut`, `Extern`),
                        decl: ExternTypeSynDecl {
                            path: TypePath(`core::mem::RefMut`, `Extern`),
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    19,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Lifetime {
                                        label_token: LifetimeToken {
                                            label: `'a`,
                                            token_idx: TokenIdx(
                                                20,
                                            ),
                                        },
                                    },
                                },
                                TemplateParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Invariant(
                                            InvariantToken {
                                                token_idx: TokenIdx(
                                                    22,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                23,
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
                                                        path: TypePath(`core::mem::RefMut`, `Extern`),
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
                                                        21,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: Some(
                                                            VarianceToken::Covariant(
                                                                CovariantToken {
                                                                    token_idx: TokenIdx(
                                                                        19,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Lifetime {
                                                            label_token: LifetimeToken {
                                                                label: `'a`,
                                                                token_idx: TokenIdx(
                                                                    20,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        24,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: Some(
                                                            VarianceToken::Invariant(
                                                                InvariantToken {
                                                                    token_idx: TokenIdx(
                                                                        22,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    23,
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
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeDefn::Extern(
                    ExternTypeSynDefn {
                        path: TypePath(`core::mem::Leash`, `Extern`),
                        decl: ExternTypeSynDecl {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    30,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                31,
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
                                                        path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                        32,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: Some(
                                                            VarianceToken::Covariant(
                                                                CovariantToken {
                                                                    token_idx: TokenIdx(
                                                                        30,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    31,
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
                                                    0..1,
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
        SynDefn::ImplBlock(
            ImplBlockSynDecl::TraitForType(
                TraitForTypeImplBlockSynDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `core::mem`,
                        trai_path: TraitPath(`core::marker::Copy`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::mem::Leash`, `Extern`),
                        ),
                        disambiguator: 0,
                    },
                    template_parameters: [
                        TemplateParameterDecl {
                            annotated_variance_token: None,
                            symbol: 0,
                            variant: TemplateParameterDeclPatternVariant::Type {
                                ident_token: IdentToken {
                                    ident: `E`,
                                    token_idx: TokenIdx(
                                        36,
                                    ),
                                },
                                traits: None,
                            },
                        },
                    ],
                    trai_expr: TraitExpr {
                        expr: 0,
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeExpr {
                            expr: 3,
                        },
                    ),
                    syn_expr_region: SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePath {
                                            path: TraitForTypeImplBlockPath {
                                                module_path: `core::mem`,
                                                trai_path: TraitPath(`core::marker::Copy`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`core::mem::Leash`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: 0,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::marker::Copy`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::mem::Leash`, `Extern`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `E`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `E`,
                                                    token_idx: TokenIdx(
                                                        36,
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
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `Copy`,
                                                token_idx: TokenIdx(
                                                    38,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::marker::Copy`),
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `Leash`,
                                                token_idx: TokenIdx(
                                                    40,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::mem::Leash`, `Extern`),
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
                                                37,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            36,
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
                                            0..1,
                                        ),
                                    ),
                                ],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: Trait,
                                    expr_idx: 0,
                                },
                                SynExprRoot {
                                    kind: SelfType,
                                    expr_idx: 3,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
    ],
)