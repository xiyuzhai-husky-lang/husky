```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Enum(
                EnumHirDecl {
                    path: TypePath(`core::result::Result`, `Enum`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `T`,
                                    traits: [],
                                },
                            },
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `E`,
                                    traits: [],
                                },
                            },
                        ],
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`core::result::Result`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `T`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 1,
                                            },
                                        ),
                                    },
                                ],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::TypeVariant(
        TypeVariantHirDecl::Tuple(
            EnumTupleVariantHirDecl {
                path: TypeVariantPath(`core::result::Result::Ok`),
                fields: [
                    EnumTupleVariantField {
                        ty: HirType::Variable(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                    },
                ],
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::ItemDecl(
                        ItemPath(`core::result::Result::Ok`),
                    ),
                    self_value_ty: None,
                    expr_arena: Arena {
                        data: [],
                    },
                    stmt_arena: Arena {
                        data: [],
                    },
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `T`,
                                    ),
                                    data: Inherited,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `E`,
                                    ),
                                    data: Inherited,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 1,
                                        },
                                    ),
                                },
                            ],
                        },
                    },
                    runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::TypeVariant(
        TypeVariantHirDecl::Tuple(
            EnumTupleVariantHirDecl {
                path: TypeVariantPath(`core::result::Result::Err`),
                fields: [
                    EnumTupleVariantField {
                        ty: HirType::Variable(
                            HirTypeTemplateVariable::Type {
                                attrs: HirTemplateVariableAttrs {
                                    class: Mono,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                    },
                ],
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::ItemDecl(
                        ItemPath(`core::result::Result::Err`),
                    ),
                    self_value_ty: None,
                    expr_arena: Arena {
                        data: [],
                    },
                    stmt_arena: Arena {
                        data: [],
                    },
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `T`,
                                    ),
                                    data: Inherited,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `E`,
                                    ),
                                    data: Inherited,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 1,
                                        },
                                    ),
                                },
                            ],
                        },
                    },
                    runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath(`core::result::Result as core::ops::Unveil(0)`),
                template_parameters: HirTemplateParameters(
                    [
                        HirTemplateParameter {
                            symbol: HirTemplateVariable::Type(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
                                        class: Mono,
                                    },
                                    variance: None,
                                    disambiguator: 0,
                                },
                            ),
                            data: HirTemplateParameterData::Type {
                                ident: `T1`,
                                traits: [],
                            },
                        },
                        HirTemplateParameter {
                            symbol: HirTemplateVariable::Type(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
                                        class: Mono,
                                    },
                                    variance: None,
                                    disambiguator: 1,
                                },
                            ),
                            data: HirTemplateParameterData::Type {
                                ident: `T2`,
                                traits: [],
                            },
                        },
                        HirTemplateParameter {
                            symbol: HirTemplateVariable::Type(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
                                        class: Mono,
                                    },
                                    variance: None,
                                    disambiguator: 2,
                                },
                            ),
                            data: HirTemplateParameterData::Type {
                                ident: `E1`,
                                traits: [],
                            },
                        },
                        HirTemplateParameter {
                            symbol: HirTemplateVariable::Type(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
                                        class: Mono,
                                    },
                                    variance: None,
                                    disambiguator: 3,
                                },
                            ),
                            data: HirTemplateParameterData::Type {
                                ident: `E2`,
                                traits: [],
                            },
                        },
                    ],
                ),
                trai: HirTrait {
                    trai_path: TraitPath(`core::ops::Unveil`),
                    template_arguments: [
                        HirTemplateArgument::Type(
                            HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::result::Result`, `Enum`),
                                    template_arguments: [
                                        HirTemplateArgument::Type(
                                            HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 1,
                                                },
                                            ),
                                        ),
                                        HirTemplateArgument::Type(
                                            HirType::Variable(
                                                HirTypeTemplateVariable::Type {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    variance: None,
                                                    disambiguator: 3,
                                                },
                                            ),
                                        ),
                                    ],
                                    always_copyable: false,
                                },
                            ),
                        ),
                    ],
                },
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`core::result::Result`, `Enum`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::Variable(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                            HirTemplateArgument::Type(
                                HirType::Variable(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: None,
                                        disambiguator: 2,
                                    },
                                ),
                            ),
                        ],
                        always_copyable: false,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::ItemDecl(
                        ItemPath(`core::result::Result as core::ops::Unveil(0)`),
                    ),
                    self_value_ty: None,
                    expr_arena: Arena {
                        data: [],
                    },
                    stmt_arena: Arena {
                        data: [],
                    },
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `T1`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `T2`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 1,
                                        },
                                    ),
                                },
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `E1`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 2,
                                        },
                                    ),
                                },
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `E2`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 3,
                                        },
                                    ),
                                },
                            ],
                        },
                    },
                    runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::AssocItem(
        AssocItemHirDecl::TraitForTypeItem(
            TraitForTypeItemHirDecl::AssocType(
                TraitForTypeAssocTypeHirDecl {
                    path: TraitForTypeItemPath(
                        `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                        TraitItemKind::AssocType,
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    ty: HirType::Variable(
                        HirTypeTemplateVariable::Type {
                            attrs: HirTemplateVariableAttrs {
                                class: Mono,
                            },
                            variance: None,
                            disambiguator: 3,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::Continue`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `T1`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `T2`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 1,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E1`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 2,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E2`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 3,
                                            },
                                        ),
                                    },
                                ],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssocItem(
        AssocItemHirDecl::TraitForTypeItem(
            TraitForTypeItemHirDecl::AssocRitchie(
                TraitForTypeAssocRitchieHirDecl {
                    path: TraitForTypeItemPath(
                        `<core::result::Result as core::ops::Unveil(0)>::unveil`,
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_idx: 0,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::result::Result`, `Enum`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::Variable(
                                                    HirTypeTemplateVariable::Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Mono,
                                                        },
                                                        variance: None,
                                                        disambiguator: 1,
                                                    },
                                                ),
                                            ),
                                            HirTemplateArgument::Type(
                                                HirType::Variable(
                                                    HirTypeTemplateVariable::Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Mono,
                                                        },
                                                        variance: None,
                                                        disambiguator: 3,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::result::Result`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::Variable(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                ),
                                HirTemplateArgument::Type(
                                    HirType::Variable(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 2,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [
                                HirEagerPatternEntry {
                                    data: HirEagerPatternData::Ident {
                                        symbol_modifier: None,
                                        ident: `result`,
                                        variable_idx: 1,
                                    },
                                    contract: Pure,
                                },
                            ],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `T1`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `T2`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 1,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E1`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 2,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E2`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Mono,
                                                },
                                                variance: None,
                                                disambiguator: 3,
                                            },
                                        ),
                                    },
                                ],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `result`,
                                        ),
                                        data: HirEagerRuntimeVariableData::ParenateParameter,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
]
```