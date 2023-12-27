[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`mnist::MnistLabel`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`mnist::MnistLabel`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::MnistLabel`, `Enum`),
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternHirDefn {
                    path: TypePath(`mnist::BinaryImage28`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`mnist::BinaryImage28`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::BinaryImage28`, `Extern`),
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternHirDefn {
                    path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist::input`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist::input`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist::input`, `Val`),
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
                    body_with_hir_expr_region: None,
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryImage28`, `Extern`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::visual::Visualize`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist::BinaryImage28`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryImage28`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `visualize`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Pure,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 82,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::visual::Html`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `mnist`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `visualize`,
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist`,
                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
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
                            ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
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
                                                        module_path: `mnist`,
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
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
            TypeItemHirDefn::AssociatedFn(
                TypeAssociatedFnHirDefn {
                    path: TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssociatedFunctionFn`),
                    hir_decl: TypeAssociatedFnHirDecl {
                        path: TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssociatedFunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssociatedFunctionFn`),
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::ops::IntIndex`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryImage28`, `Extern`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::ops::IntIndex`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist`,
                                            trai_path: TraitPath(`core::ops::IntIndex`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist::BinaryImage28`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssociatedType(
                TraitForTypeAssociatedTypeHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist`,
                                                trai_path: TraitPath(`core::ops::IntIndex`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryImage28`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `Output`,
                                        item_kind: AssociatedType,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeAssociatedTypeHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `Output`,
                                            item_kind: AssociatedType,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `mnist`,
                                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `Output`,
                                                            item_kind: AssociatedType,
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
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::visual::Visualize`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `visualize`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Pure,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 83,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::visual::Html`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `mnist`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `visualize`,
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist`,
                                        ty_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
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
                            ty_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
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
                                                        module_path: `mnist`,
                                                        ty_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
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
            TypeItemHirDefn::AssociatedFn(
                TypeAssociatedFnHirDefn {
                    path: TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
                    hir_decl: TypeAssociatedFnHirDecl {
                        path: TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::ops::IntIndex`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::ops::IntIndex`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist`,
                                            trai_path: TraitPath(`core::ops::IntIndex`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssociatedType(
                TraitForTypeAssociatedTypeHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist`,
                                                trai_path: TraitPath(`core::ops::IntIndex`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `Output`,
                                        item_kind: AssociatedType,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeAssociatedTypeHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `Output`,
                                            item_kind: AssociatedType,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `mnist`,
                                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `Output`,
                                                            item_kind: AssociatedType,
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
    ),
]