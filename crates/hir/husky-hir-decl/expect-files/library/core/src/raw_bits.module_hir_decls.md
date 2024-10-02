```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`core::raw_bits::r32`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`core::raw_bits::r32`),
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
            ),
        ),
    ),
    HirDecl::Attr(
        AttrHirDecl::Derive(
            DeriveAttrHirDecl {
                path: AttrItemPath(`core::raw_bits::r32::#derive(0)`),
                trais: [
                    HirTrait {
                        trai_path: TraitPath(`core::fmt::Debug`),
                        template_arguments: [],
                    },
                    HirTrait {
                        trai_path: TraitPath(`core::cmp::PartialEq`),
                        template_arguments: [],
                    },
                    HirTrait {
                        trai_path: TraitPath(`core::cmp::Eq`),
                        template_arguments: [],
                    },
                    HirTrait {
                        trai_path: TraitPath(`core::clone::Clone`),
                        template_arguments: [],
                    },
                    HirTrait {
                        trai_path: TraitPath(`core::marker::Copy`),
                        template_arguments: [],
                    },
                ],
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::ItemDecl(
                        ItemPath(`core::raw_bits::r32::#derive(0)`),
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
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath(`core::raw_bits::r32(0)`),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                        template_arguments: [],
                        always_copyable: true,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::ItemDecl(
                        ItemPath(`core::raw_bits::r32(0)`),
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
        ),
    ),
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodRitchieHirDecl {
                    path: TypeItemPath(
                        `core::raw_bits::r32(0)::last_bits`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_idx: 0,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`core::raw_bits::r32(0)::last_bits`),
                        ),
                        self_value_ty: Some(
                            HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
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
                                        ident: `k`,
                                        variable_idx: 1,
                                    },
                                    contract: Pure,
                                },
                            ],
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
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `k`,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodRitchieHirDecl {
                    path: TypeItemPath(
                        `core::raw_bits::r32(0)::ctz`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`core::raw_bits::r32(0)::ctz`),
                        ),
                        self_value_ty: Some(
                            HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
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
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodRitchieHirDecl {
                    path: TypeItemPath(
                        `core::raw_bits::r32(0)::co`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`core::raw_bits::r32(0)::co`),
                        ),
                        self_value_ty: Some(
                            HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
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
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodRitchieHirDecl {
                    path: TypeItemPath(
                        `core::raw_bits::r32(0)::span`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`core::raw_bits::r32(0)::span`),
                        ),
                        self_value_ty: Some(
                            HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
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
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodRitchieHirDecl {
                    path: TypeItemPath(
                        `core::raw_bits::r32(0)::right_mass`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`core::raw_bits::r32(0)::right_mass`),
                        ),
                        self_value_ty: Some(
                            HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
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
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
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