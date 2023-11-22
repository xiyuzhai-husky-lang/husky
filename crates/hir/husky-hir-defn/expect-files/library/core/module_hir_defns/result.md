[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`core::result::Result`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`core::result::Result`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 115,
                                                },
                                            ),
                                        ),
                                        traits: [],
                                    },
                                },
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 1,
                                        },
                                    ),
                                    data: Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 25,
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
                                        TypePath(`core::result::Result`, `Enum`),
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
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `T`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `E`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 1,
                                                },
                                            ),
                                        },
                                    ],
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
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::result::Result`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: Type(
                                    Type {
                                        attrs: HirSymbolAttrs,
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: Type {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 136,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            },
                            HirTemplateParameter {
                                symbol: Type(
                                    Type {
                                        attrs: HirSymbolAttrs,
                                        variance: None,
                                        disambiguator: 1,
                                    },
                                ),
                                data: Type {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 137,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            },
                            HirTemplateParameter {
                                symbol: Type(
                                    Type {
                                        attrs: HirSymbolAttrs,
                                        variance: None,
                                        disambiguator: 2,
                                    },
                                ),
                                data: Type {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    traits: [],
                                },
                            },
                            HirTemplateParameter {
                                symbol: Type(
                                    Type {
                                        attrs: HirSymbolAttrs,
                                        variance: None,
                                        disambiguator: 3,
                                    },
                                ),
                                data: Type {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 139,
                                            },
                                        ),
                                    ),
                                    traits: [],
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
                                        ty_path: TypePath(`core::result::Result`, `Enum`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::Symbol(
                                                    Type {
                                                        attrs: HirSymbolAttrs,
                                                        variance: None,
                                                        disambiguator: 1,
                                                    },
                                                ),
                                            ),
                                            HirTemplateArgument::Type(
                                                HirType::Symbol(
                                                    Type {
                                                        attrs: HirSymbolAttrs,
                                                        variance: None,
                                                        disambiguator: 3,
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                ),
                            ),
                        ],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::result::Result`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::Symbol(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                ),
                                HirTemplateArgument::Type(
                                    HirType::Symbol(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 2,
                                        },
                                    ),
                                ),
                            ],
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
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
                                data: [
                                    HirEagerComptimeSymbolEntry {
                                        name: HirEagerComptimeSymbolName::Ident(
                                            `T1`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirComptimeSymbol::Type(
                                            Type {
                                                attrs: HirSymbolAttrs,
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeSymbolEntry {
                                        name: HirEagerComptimeSymbolName::Ident(
                                            `T2`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirComptimeSymbol::Type(
                                            Type {
                                                attrs: HirSymbolAttrs,
                                                variance: None,
                                                disambiguator: 1,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeSymbolEntry {
                                        name: HirEagerComptimeSymbolName::Ident(
                                            `E1`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirComptimeSymbol::Type(
                                            Type {
                                                attrs: HirSymbolAttrs,
                                                variance: None,
                                                disambiguator: 2,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeSymbolEntry {
                                        name: HirEagerComptimeSymbolName::Ident(
                                            `E2`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirComptimeSymbol::Type(
                                            Type {
                                                attrs: HirSymbolAttrs,
                                                variance: None,
                                                disambiguator: 3,
                                            },
                                        ),
                                    },
                                ],
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
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssociatedType(
                TraitForTypeAssociatedTypeHirDefn {
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `Continue`,
                        item_kind: AssociatedType,
                    },
                    hir_decl: TraitForTypeAssociatedTypeHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `core::result`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`core::result::Result`, `Enum`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `Continue`,
                            item_kind: AssociatedType,
                        },
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        associated_ty: HirType::Symbol(
                            Type {
                                attrs: HirSymbolAttrs,
                                variance: None,
                                disambiguator: 3,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                            ident: `Continue`,
                                            item_kind: AssociatedType,
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
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `T1`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `T2`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 1,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `E1`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 2,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `E2`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 3,
                                                },
                                            ),
                                        },
                                    ],
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `branch`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `core::result`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`core::result::Result`, `Enum`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `branch`,
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
                                            ty_path: TypePath(`core::result::Result`, `Enum`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Symbol(
                                                        Type {
                                                            attrs: HirSymbolAttrs,
                                                            variance: None,
                                                            disambiguator: 1,
                                                        },
                                                    ),
                                                ),
                                                HirTemplateArgument::Type(
                                                    HirType::Symbol(
                                                        Type {
                                                            attrs: HirSymbolAttrs,
                                                            variance: None,
                                                            disambiguator: 3,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::result::Result`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::Symbol(
                                            Type {
                                                attrs: HirSymbolAttrs,
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    ),
                                    HirTemplateArgument::Type(
                                        HirType::Symbol(
                                            Type {
                                                attrs: HirSymbolAttrs,
                                                variance: None,
                                                disambiguator: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `core::result`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                            ident: `branch`,
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
                                                    value: 17,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `T1`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `T2`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 1,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `E1`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 2,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSymbolEntry {
                                            name: HirEagerComptimeSymbolName::Ident(
                                                `E2`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirComptimeSymbol::Type(
                                                Type {
                                                    attrs: HirSymbolAttrs,
                                                    variance: None,
                                                    disambiguator: 3,
                                                },
                                            ),
                                        },
                                    ],
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
                                                `result`,
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
                            2,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath {
                                                impl_block: TraitForTypeImplBlockPath {
                                                    module_path: `core::result`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                                ident: `branch`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Todo,
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 1,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `T1`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirComptimeSymbol::Type(
                                                    Type {
                                                        attrs: HirSymbolAttrs,
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            },
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `T2`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirComptimeSymbol::Type(
                                                    Type {
                                                        attrs: HirSymbolAttrs,
                                                        variance: None,
                                                        disambiguator: 1,
                                                    },
                                                ),
                                            },
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `E1`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirComptimeSymbol::Type(
                                                    Type {
                                                        attrs: HirSymbolAttrs,
                                                        variance: None,
                                                        disambiguator: 2,
                                                    },
                                                ),
                                            },
                                            HirEagerComptimeSymbolEntry {
                                                name: HirEagerComptimeSymbolName::Ident(
                                                    `E2`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirComptimeSymbol::Type(
                                                    Type {
                                                        attrs: HirSymbolAttrs,
                                                        variance: None,
                                                        disambiguator: 3,
                                                    },
                                                ),
                                            },
                                        ],
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
                                                    `result`,
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
                        ),
                    ),
                },
            ),
        ),
    ),
]