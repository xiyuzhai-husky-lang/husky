```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`core::result::Result`, `Enum`),
                    hir_decl: EnumHirDecl {
                        path: TypePath(`core::result::Result`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Comptime,
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
                                                class: Comptime,
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
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::result::Result`, `Enum`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
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
                                                        class: Comptime,
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
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 1,
                                                },
                                            ),
                                        },
                                    ],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Tuple(
            EnumTupleVariantHirDefn {
                path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 91,
                        },
                    ),
                ),
                hir_decl: EnumTupleVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 91,
                            },
                        ),
                    ),
                    fields: [
                        EnumTupleVariantField {
                            ty: HirType::Variable(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
                                        class: Comptime,
                                    },
                                    variance: None,
                                    disambiguator: 0,
                                },
                            ),
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::Decl(
                            ItemPath::TypeVariant(
                                Room32,
                                TypeVariantPath(
                                    ItemPathId(
                                        Id {
                                            value: 91,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
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
                                                    class: Comptime,
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
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 1,
                                            },
                                        ),
                                    },
                                ],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Tuple(
            EnumTupleVariantHirDefn {
                path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                hir_decl: EnumTupleVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    fields: [
                        EnumTupleVariantField {
                            ty: HirType::Variable(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
                                        class: Comptime,
                                    },
                                    variance: None,
                                    disambiguator: 1,
                                },
                            ),
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::Decl(
                            ItemPath::TypeVariant(
                                Room32,
                                TypeVariantPath(
                                    ItemPathId(
                                        Id {
                                            value: 92,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
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
                                                    class: Comptime,
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
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 1,
                                            },
                                        ),
                                    },
                                ],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlockPath(`core::result::Result as core::ops::Unveil(0)`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Comptime,
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
                                            class: Comptime,
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
                                            class: Comptime,
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
                                            class: Comptime,
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
                                                            class: Comptime,
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
                                                            class: Comptime,
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
                                                class: Comptime,
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
                                                class: Comptime,
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
                        region_path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockPath(`core::result::Result as core::ops::Unveil(0)`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
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
                                                    class: Comptime,
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
                                                    class: Comptime,
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
                                                    class: Comptime,
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
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 3,
                                            },
                                        ),
                                    },
                                ],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssocType(
                TraitForTypeAssocTypeHirDefn {
                    path: TraitForTypeItemPath(
                        `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                        TraitItemKind::AssocType,
                    ),
                    hir_decl: TraitForTypeAssocTypeHirDecl {
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
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 3,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                                            TraitItemKind::AssocType,
                                        ),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
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
                                                        class: Comptime,
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
                                                        class: Comptime,
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
                                                        class: Comptime,
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
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 3,
                                                },
                                            ),
                                        },
                                    ],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssocRitchie(
                TraitForTypeAssocRitchieHirDefn {
                    path: TraitForTypeItemPath(
                        `<core::result::Result as core::ops::Unveil(0)>::unveil`,
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TraitForTypeAssocRitchieHirDecl {
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
                                                                class: Comptime,
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
                                                                class: Comptime,
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
                                                    class: Comptime,
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
                                                    class: Comptime,
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
                            region_path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            `<core::result::Result as core::ops::Unveil(0)>::unveil`,
                                            TraitItemKind::AssocRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                ),
                            ),
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
                                        },
                                        contract: Pure,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
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
                                                        class: Comptime,
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
                                                        class: Comptime,
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
                                                        class: Comptime,
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
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 3,
                                                },
                                            ),
                                        },
                                    ],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
]
```