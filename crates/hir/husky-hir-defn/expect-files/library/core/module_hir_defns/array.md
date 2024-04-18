```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternHirDefn {
                    path: TypePath(`core::array::Array`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::array::Array`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: HirTemplateVariable::Const(
                                        HirConstTemplateVariable {
                                            ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::usize`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            index: HirConstTemplateVariableIndex::PathLeading {
                                                attrs: HirTemplateVariableAttrs {
                                                    class: Comptime,
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
                                                class: Comptime,
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
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::array::Array`, `Extern`),
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
                                                `L`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateVariable::Const(
                                                HirConstTemplateVariable {
                                                    ty: HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`core::num::usize`, `Extern`),
                                                            template_arguments: [],
                                                            always_copyable: true,
                                                        },
                                                    ),
                                                    index: HirConstTemplateVariableIndex::PathLeading {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
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
                                                        class: Comptime,
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
                },
            ),
        ),
    ),
]
```