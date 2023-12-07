[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            PropsStruct(
                PropsStructTypeHirDecl(
                    Id {
                        value: 4,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Ordinary {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                            },
                            HirEagerParenateParameter::Ordinary {
                                pattern_expr_idx: 2,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::Ritchie(
                                                    HirRitchieType {
                                                        ritchie_ty_kind: Fn,
                                                        parameters: HirRitchieParameters {
                                                            data: [
                                                                HirRitchieParameter::Regular(
                                                                    HirRitchieRegularParameter {
                                                                        contract: Pure,
                                                                        ty: HirType::PathLeading(
                                                                            HirTypePathLeading {
                                                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                                template_arguments: [
                                                                                    HirTemplateArgument::Type(
                                                                                        HirType::PathLeading(
                                                                                            HirTypePathLeading {
                                                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                                template_arguments: [],
                                                                                                always_copyable: false,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ],
                                                                                always_copyable: true,
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        return_ty: HirType::PathLeading(
                                                            HirTypePathLeading {
                                                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                                                template_arguments: [
                                                                    HirTemplateArgument::Type(
                                                                        HirType::PathLeading(
                                                                            HirTypePathLeading {
                                                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                                                template_arguments: [],
                                                                                always_copyable: true,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ],
                                                                always_copyable: true,
                                                            },
                                                        ),
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
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    symbol_modifier: None,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 368,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    symbol_modifier: None,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 430,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::Ident(
                                            `concave_components`,
                                        ),
                                        data: HirEagerRuntimeSymbolData::ParenateParameter,
                                    },
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::Ident(
                                            `templates`,
                                        ),
                                        data: HirEagerRuntimeSymbolData::ParenateParameter,
                                    },
                                ],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        template_arguments: [],
                        always_copyable: false,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TypeImplBlock(
                                TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist_classifier::fermi`,
                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
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
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                        arena: Arena {
                            data: [],
                        },
                    },
                    runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
]