```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    hir_decl: EnumHirDecl {
                        path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
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
                                    data: [],
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
        TypeVariantHirDefn::Unit(
            EnumUnitVariantHirDefn {
                path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                hir_decl: EnumUnitTypeVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
                                data: [],
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
        TypeVariantHirDefn::Props(
            EnumPropsVariantHirDefn {
                path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 11,
                        },
                    ),
                ),
                hir_decl: EnumPropsVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    fields: [
                        EnumPropsVariantField {
                            ident: `x`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `x`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
                                    },
                                ],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Props(
            EnumPropsVariantHirDefn {
                path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 12,
                        },
                    ),
                ),
                hir_decl: EnumPropsVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 12,
                            },
                        ),
                    ),
                    fields: [
                        EnumPropsVariantField {
                            ident: `x`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                        },
                        EnumPropsVariantField {
                            ident: `y`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `x`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `y`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
                                    },
                                ],
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
                            value: 13,
                        },
                    ),
                ),
                hir_decl: EnumTupleVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    fields: [
                        EnumTupleVariantField {
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
                                data: [],
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
                            value: 14,
                        },
                    ),
                ),
                hir_decl: EnumTupleVariantHirDecl {
                    path: TypeVariantPath(
                        ItemPathId(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    fields: [
                        EnumTupleVariantField {
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                        },
                        EnumTupleVariantField {
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
                                data: [],
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