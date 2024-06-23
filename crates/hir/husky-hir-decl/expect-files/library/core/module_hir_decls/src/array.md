```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`core::array::Array`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Compterm(
                                    HirComptermTemplateVariable {
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::usize`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                        index: HirComptermTemplateVariableIndex::PathLeading {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            disambiguator: 0,
                                            ty_path: TypePath(`core::num::usize`, `Extern`),
                                        },
                                    },
                                ),
                                data: HirTemplateParameterData::Constant {
                                    ident: `L`,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::usize`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            },
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
                                            class: Mono,
                                        },
                                        variance: Some(
                                            Covariant,
                                        ),
                                        disambiguator: 0,
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
                            ItemPath(`core::array::Array`),
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
                                            `L`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateVariable::Compterm(
                                            HirComptermTemplateVariable {
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                index: HirComptermTemplateVariableIndex::PathLeading {
                                                    attrs: HirTemplateVariableAttrs {
                                                        class: Mono,
                                                    },
                                                    disambiguator: 0,
                                                    ty_path: TypePath(`core::num::usize`, `Extern`),
                                                },
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
                                                variance: Some(
                                                    Covariant,
                                                ),
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
            ),
        ),
    ),
]
```