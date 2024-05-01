```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`core::option::Option`, `Enum`),
                    hir_decl: EnumHirDecl {
                        path: TypePath(`core::option::Option`, `Enum`),
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
                            ],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::option::Option`, `Enum`),
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
                            value: 78,
                        },
                    ),
                ),
                hir_decl: EnumTupleVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 78,
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
                                            value: 78,
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
        TypeVariantHirDefn::Unit(
            EnumUnitVariantHirDefn {
                path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 79,
                        },
                    ),
                ),
                hir_decl: EnumUnitTypeVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 79,
                            },
                        ),
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::Decl(
                            ItemPath::TypeVariant(
                                Room32,
                                TypeVariantPath(
                                    ItemPathId(
                                        Id {
                                            value: 79,
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
]
```