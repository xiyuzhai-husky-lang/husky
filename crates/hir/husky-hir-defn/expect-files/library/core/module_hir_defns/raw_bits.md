[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternHirDefn {
                    path: TypePath(`core::raw_bits::r32`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::raw_bits::r32`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::raw_bits::r32`, `Extern`),
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
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `core::raw_bits`,
                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
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
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            template_arguments: [],
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
                                                        module_path: `core::raw_bits`,
                                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
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
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TypeItem(
                                    TypeItemPathData {
                                        impl_block: TypeImplBlockPath(
                                            ItemPathId {
                                                data: ItemPathData::ImplBlock(
                                                    ImplBlockPathData::TypeImplBlock(
                                                        TypeImplBlockPathData {
                                                            module_path: `core::raw_bits`,
                                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ident: `last_bits`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TypeItem(
                                        TypeItemPathData {
                                            impl_block: TypeImplBlockPath(
                                                ItemPathId {
                                                    data: ItemPathData::ImplBlock(
                                                        ImplBlockPathData::TypeImplBlock(
                                                            TypeImplBlockPathData {
                                                                module_path: `core::raw_bits`,
                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            ident: `last_bits`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::raw_bits`,
                                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `last_bits`,
                                                            item_kind: MethodFn,
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
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 128,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `k`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TypeItem(
                                    TypeItemPathData {
                                        impl_block: TypeImplBlockPath(
                                            ItemPathId {
                                                data: ItemPathData::ImplBlock(
                                                    ImplBlockPathData::TypeImplBlock(
                                                        TypeImplBlockPathData {
                                                            module_path: `core::raw_bits`,
                                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ident: `ctz`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TypeItem(
                                        TypeItemPathData {
                                            impl_block: TypeImplBlockPath(
                                                ItemPathId {
                                                    data: ItemPathData::ImplBlock(
                                                        ImplBlockPathData::TypeImplBlock(
                                                            TypeImplBlockPathData {
                                                                module_path: `core::raw_bits`,
                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            ident: `ctz`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::raw_bits`,
                                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `ctz`,
                                                            item_kind: MethodFn,
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
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TypeItem(
                                    TypeItemPathData {
                                        impl_block: TypeImplBlockPath(
                                            ItemPathId {
                                                data: ItemPathData::ImplBlock(
                                                    ImplBlockPathData::TypeImplBlock(
                                                        TypeImplBlockPathData {
                                                            module_path: `core::raw_bits`,
                                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ident: `co`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TypeItem(
                                        TypeItemPathData {
                                            impl_block: TypeImplBlockPath(
                                                ItemPathId {
                                                    data: ItemPathData::ImplBlock(
                                                        ImplBlockPathData::TypeImplBlock(
                                                            TypeImplBlockPathData {
                                                                module_path: `core::raw_bits`,
                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            ident: `co`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::raw_bits`,
                                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `co`,
                                                            item_kind: MethodFn,
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
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TypeItem(
                                    TypeItemPathData {
                                        impl_block: TypeImplBlockPath(
                                            ItemPathId {
                                                data: ItemPathData::ImplBlock(
                                                    ImplBlockPathData::TypeImplBlock(
                                                        TypeImplBlockPathData {
                                                            module_path: `core::raw_bits`,
                                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ident: `span`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TypeItem(
                                        TypeItemPathData {
                                            impl_block: TypeImplBlockPath(
                                                ItemPathId {
                                                    data: ItemPathData::ImplBlock(
                                                        ImplBlockPathData::TypeImplBlock(
                                                            TypeImplBlockPathData {
                                                                module_path: `core::raw_bits`,
                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            ident: `span`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::raw_bits`,
                                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `span`,
                                                            item_kind: MethodFn,
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
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TypeItem(
                                    TypeItemPathData {
                                        impl_block: TypeImplBlockPath(
                                            ItemPathId {
                                                data: ItemPathData::ImplBlock(
                                                    ImplBlockPathData::TypeImplBlock(
                                                        TypeImplBlockPathData {
                                                            module_path: `core::raw_bits`,
                                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ident: `right_mass`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TypeItem(
                                        TypeItemPathData {
                                            impl_block: TypeImplBlockPath(
                                                ItemPathId {
                                                    data: ItemPathData::ImplBlock(
                                                        ImplBlockPathData::TypeImplBlock(
                                                            TypeImplBlockPathData {
                                                                module_path: `core::raw_bits`,
                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            ident: `right_mass`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::raw_bits`,
                                                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `right_mass`,
                                                            item_kind: MethodFn,
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
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
]