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
                                    data: [],
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
                },
            ),
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Unit(
            EnumUnitVariantHirDefn {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
                hir_decl: EnumUnitTypeVariantHirDecl {
                    path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
                                data: [],
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
            },
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Props(
            EnumPropsVariantHirDefn {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                hir_decl: EnumPropsVariantHirDecl {
                    path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                hir_decl: EnumPropsVariantHirDecl {
                    path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                hir_decl: EnumTupleVariantHirDecl {
                    path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
                                data: [],
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
            },
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Tuple(
            EnumTupleVariantHirDefn {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                hir_decl: EnumTupleVariantHirDecl {
                    path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
                                data: [],
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
            },
        ),
    ),
]
```