[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`malamute::Class`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`malamute::Class`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 534,
                                                },
                                            ),
                                        ),
                                        traits: [],
                                    },
                                },
                            ],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::Class`, `Enum`),
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
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `Label`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateSymbol::Type(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
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
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::OneVsAll`, `Enum`),
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
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
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
            FugitiveHirDefn::FunctionGn(
                FunctionGnHirDefn {
                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                    hir_decl: FunctionGnFugitiveHirDecl {
                        path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 534,
                                                },
                                            ),
                                        ),
                                        traits: [],
                                    },
                                },
                                HirTemplateParameter {
                                    symbol: Const(
                                        HirConstSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                    data: Constant {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 537,
                                                },
                                            ),
                                        ),
                                        ty: Symbol(
                                            Type {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                },
                            ],
                        ),
                        parenate_parameters: HirLazyParenateParameters(
                            [
                                Variadic {
                                    variant: Vec,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                },
                                Keyed {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 442,
                                            },
                                        ),
                                    ),
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_lazy_expr_region: HirLazyExprRegion {
                            hir_lazy_expr_arena: Arena {
                                data: [],
                            },
                            hir_lazy_stmt_arena: Arena {
                                data: [],
                            },
                            hir_lazy_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_lazy_variable_region: HirLazyVariableRegion {
                                arena: Arena {
                                    data: [
                                        HirLazyVariable {
                                            name: Ident(
                                                Ident(
                                                    Coword(
                                                        Id {
                                                            value: 146,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            data: ParenateParameter,
                                        },
                                        HirLazyVariable {
                                            name: Ident(
                                                Ident(
                                                    Coword(
                                                        Id {
                                                            value: 442,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            data: ParenateParameter,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                    lazy_body_with_hir_lazy_expr_region: None,
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
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::default::Default`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::default::Default`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::default::Default`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
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
            TraitForTypeItemHirDefn::AssociatedFn(
                TraitForTypeAssociatedFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::default::Default`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `default`,
                                        item_kind: AssociatedFunctionFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeAssociatedFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::default::Default`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `default`,
                                            item_kind: AssociatedFunctionFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::default::Default`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `default`,
                                                            item_kind: AssociatedFunctionFn,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            2,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::default::Default`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `default`,
                                                                item_kind: AssociatedFunctionFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ident: `No`,
                                                                    index: U8(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 1,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
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
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: Type(
                                    Type {
                                        attrs: HirTemplateSymbolAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: Type {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 534,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            },
                            HirTemplateParameter {
                                symbol: Const(
                                    HirConstSymbol(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                                data: Constant {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 537,
                                            },
                                        ),
                                    ),
                                    ty: Symbol(
                                        Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                            },
                        ],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::ops::Unveil`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            ),
                        ],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`malamute::Class`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::Symbol(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::Class`, `Enum`),
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
                                data: [
                                    HirEagerComptimeSymbolEntry {
                                        name: HirEagerComptimeSymbolName::Ident(
                                            `Label`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateSymbol::Type(
                                            HirTypeSymbol::Type {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeSymbolEntry {
                                        name: HirEagerComptimeSymbolName::Ident(
                                            `label`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateSymbol::Const(
                                            HirConstSymbol {
                                                ty: HirType::Symbol(
                                                    HirTypeSymbol::Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                index: Other {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Runtime,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ],
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
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::Class`, `Enum`),
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
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::Class`, `Enum`),
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
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
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
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::Class`, `Enum`),
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
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `Label`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirTemplateSymbol::Type(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `label`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirTemplateSymbol::Const(
                                                HirConstSymbol {
                                                    ty: HirType::Symbol(
                                                        HirTypeSymbol::Type {
                                                            attrs: HirTemplateSymbolAttrs {
                                                                class: Comptime,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                    index: Other {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Runtime,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        },
                                    ],
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssociatedFn(
                TraitForTypeAssociatedFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::Class`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `unveil`,
                                        item_kind: AssociatedFunctionFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeAssociatedFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::Class`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `unveil`,
                                            item_kind: AssociatedFunctionFn,
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                                template_arguments: [
                                                    HirTemplateArgument::Type(
                                                        HirType::Symbol(
                                                            HirTypeSymbol::Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
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
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::Class`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `unveil`,
                                                            item_kind: AssociatedFunctionFn,
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
                                                    value: 540,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `Label`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirTemplateSymbol::Type(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `label`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirTemplateSymbol::Const(
                                                HirConstSymbol {
                                                    ty: HirType::Symbol(
                                                        HirTypeSymbol::Type {
                                                            attrs: HirTemplateSymbolAttrs {
                                                                class: Comptime,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                    index: Other {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Runtime,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        },
                                    ],
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
                                                `one_vs_all`,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::Class`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `unveil`,
                                                                item_kind: AssociatedFunctionFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::ConstSymbol {
                                                ident: `label`,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                                                ident: `Known`,
                                                                index: U8(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: Symbol(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                                ident: `Break`,
                                                                index: U8(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 79,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        3,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::Unit(
                                                    (),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                                ident: `Continue`,
                                                                index: U8(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 1,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 4,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 6,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 1,
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 2,
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                            ],
                                            match_target: 1,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 513,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 514,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `Label`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateSymbol::Type(
                                                    HirTypeSymbol::Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            },
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `label`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateSymbol::Const(
                                                    HirConstSymbol {
                                                        ty: HirType::Symbol(
                                                            HirTypeSymbol::Type {
                                                                attrs: HirTemplateSymbolAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        index: Other {
                                                            attrs: HirTemplateSymbolAttrs {
                                                                class: Runtime,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `one_vs_all`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
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
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::ops::Unveil`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            ),
                        ],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
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
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
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
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssociatedFn(
                TraitForTypeAssociatedFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `unveil`,
                                        item_kind: AssociatedFunctionFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeAssociatedFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `unveil`,
                                            item_kind: AssociatedFunctionFn,
                                        },
                                    ),
                                ),
                            },
                        ),
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
                                            ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
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
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `unveil`,
                                                            item_kind: AssociatedFunctionFn,
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
                                                    value: 545,
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
                                                `one_vs_all_result`,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `unveil`,
                                                                item_kind: AssociatedFunctionFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ident: `Yes`,
                                                                    index: U8(
                                                                        0,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                                ident: `Break`,
                                                                index: U8(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 78,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ident: `No`,
                                                                    index: U8(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                                ident: `Break`,
                                                                index: U8(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 78,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::Unit(
                                                    (),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                                ident: `Continue`,
                                                                index: U8(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 1,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        6,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    4..5,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 5,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 7,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 1,
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 2,
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 3,
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ],
                                            match_target: 1,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 516,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 517,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Unit(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 518,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
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
                                                    `one_vs_all_result`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]