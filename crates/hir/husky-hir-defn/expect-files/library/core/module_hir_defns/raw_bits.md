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
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeImplBlockPath {
                        module_path: `core::raw_bits`,
                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                        disambiguator: 0,
                    },
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
                                    TypeImplBlockPath {
                                        module_path: `core::raw_bits`,
                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        ),
                        hir_eager_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_stmt_arena: Arena {
                            data: [],
                        },
                        hir_eager_pattern_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::raw_bits`,
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `last_bits`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `last_bits`,
                            item_kind: MethodFn,
                        },
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
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `last_bits`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::raw_bits`,
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `ctz`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `ctz`,
                            item_kind: MethodFn,
                        },
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
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `ctz`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::raw_bits`,
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `co`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `co`,
                            item_kind: MethodFn,
                        },
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
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `co`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::raw_bits`,
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `span`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `span`,
                            item_kind: MethodFn,
                        },
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
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `span`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `core::raw_bits`,
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            disambiguator: 0,
                        },
                        ident: `right_mass`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `core::raw_bits`,
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                disambiguator: 0,
                            },
                            ident: `right_mass`,
                            item_kind: MethodFn,
                        },
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
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `core::raw_bits`,
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                            ident: `right_mass`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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