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
                                    symbol: HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: HirTemplateParameterData::Type {
                                        ident: `Label`,
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `Label`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateVar::Type(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateSvarAttrs {
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                        Gn,
                    )`),
                    hir_decl: FunctionGnFugitiveHirDecl {
                        path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                            Gn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: HirTemplateParameterData::Type {
                                        ident: `Label`,
                                        traits: [],
                                    },
                                },
                                HirTemplateParameter {
                                    symbol: HirTemplateVar::Const(
                                        HirConstSvar {
                                            ty: HirType::Svar(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateSvarAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: HirConstSvarIndex::Other {
                                                attrs: HirTemplateSvarAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    data: HirTemplateParameterData::Constant {
                                        ident: `label`,
                                        ty: HirType::Svar(
                                            HirTypeSvar::Type {
                                                attrs: HirTemplateSvarAttrs {
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
                                                value: 2,
                                            },
                                        ),
                                    ),
                                },
                                Keyed {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 36,
                                            },
                                        ),
                                    ),
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 3,
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
                                                            value: 34,
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
                                                            value: 36,
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssocFn(
                TraitForTypeAssocFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 147,
                            },
                        ),
                    ),
                    hir_decl: TraitForTypeAssocFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 147,
                                },
                            ),
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
                                ItemPath::AssocItem(
                                    AssocItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 147,
                                                },
                                            ),
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
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
                                    ItemPath::AssocItem(
                                        AssocItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 147,
                                                    },
                                                ),
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
                                                        ItemPathId(
                                                            Id {
                                                                value: 6,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
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
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
                                symbol: HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateSvarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `Label`,
                                    traits: [],
                                },
                            },
                            HirTemplateParameter {
                                symbol: HirTemplateVar::Const(
                                    HirConstSvar {
                                        ty: HirType::Svar(
                                            HirTypeSvar::Type {
                                                attrs: HirTemplateSvarAttrs {
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                        index: HirConstSvarIndex::Other {
                                            attrs: HirTemplateSvarAttrs {
                                                class: Runtime,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                data: HirTemplateParameterData::Constant {
                                    ident: `label`,
                                    ty: HirType::Svar(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
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
                                    HirType::Svar(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeSvarEntry {
                                        name: HirEagerComptimeSvarName::Ident(
                                            `Label`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateVar::Type(
                                            HirTypeSvar::Type {
                                                attrs: HirTemplateSvarAttrs {
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                    HirEagerComptimeSvarEntry {
                                        name: HirEagerComptimeSvarName::Ident(
                                            `label`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateVar::Const(
                                            HirConstSvar {
                                                ty: HirType::Svar(
                                                    HirTypeSvar::Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                index: HirConstSvarIndex::Other {
                                                    attrs: HirTemplateSvarAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssocType(
                TraitForTypeAssocTypeHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 148,
                            },
                        ),
                    ),
                    hir_decl: TraitForTypeAssocTypeHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 148,
                                },
                            ),
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
                                ItemPath::AssocItem(
                                    AssocItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 148,
                                                },
                                            ),
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `Label`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirTemplateVar::Type(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateSvarAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `label`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirTemplateVar::Const(
                                                HirConstSvar {
                                                    ty: HirType::Svar(
                                                        HirTypeSvar::Type {
                                                            attrs: HirTemplateSvarAttrs {
                                                                class: Comptime,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                    index: HirConstSvarIndex::Other {
                                                        attrs: HirTemplateSvarAttrs {
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssocFn(
                TraitForTypeAssocFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 149,
                            },
                        ),
                    ),
                    hir_decl: TraitForTypeAssocFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 149,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
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
                                                        HirType::Svar(
                                                            HirTypeSvar::Type {
                                                                attrs: HirTemplateSvarAttrs {
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
                                ItemPath::AssocItem(
                                    AssocItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 149,
                                                },
                                            ),
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
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `one_vs_all`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `Label`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirTemplateVar::Type(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateSvarAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        },
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `label`,
                                            ),
                                            data: Inherited,
                                            hir_comptime_symbol: HirTemplateVar::Const(
                                                HirConstSvar {
                                                    ty: HirType::Svar(
                                                        HirTypeSvar::Type {
                                                            attrs: HirTemplateSvarAttrs {
                                                                class: Comptime,
                                                            },
                                                            variance: None,
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                    index: HirConstSvarIndex::Other {
                                                        attrs: HirTemplateSvarAttrs {
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `one_vs_all`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
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
                                    ItemPath::AssocItem(
                                        AssocItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 149,
                                                    },
                                                ),
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
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::ConstSvar {
                                                ident: `label`,
                                            },
                                            ty_place: Const,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::Svar(
                                                                        HirTypeSvar::Type {
                                                                            attrs: HirTemplateSvarAttrs {
                                                                                class: Comptime,
                                                                            },
                                                                            variance: None,
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: Svar(
                                                                Type {
                                                                    attrs: HirTemplateSvarAttrs {
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
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 101,
                                                        },
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`malamute::Class`, `Enum`),
                                                                            template_arguments: [
                                                                                HirTemplateArgument::Type(
                                                                                    HirType::Svar(
                                                                                        HirTypeSvar::Type {
                                                                                            attrs: HirTemplateSvarAttrs {
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
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 1,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                                            template_arguments: [],
                                                                            always_copyable: true,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 6,
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
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::Unit(
                                                    (),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 100,
                                                        },
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`malamute::Class`, `Enum`),
                                                                            template_arguments: [
                                                                                HirTemplateArgument::Type(
                                                                                    HirType::Svar(
                                                                                        HirTypeSvar::Type {
                                                                                            attrs: HirTemplateSvarAttrs {
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
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 1,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                                            template_arguments: [],
                                                                            always_copyable: true,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
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
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
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
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 5,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 6,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerComptimeSvarEntry {
                                                name: HirEagerComptimeSvarName::Ident(
                                                    `Label`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateVar::Type(
                                                    HirTypeSvar::Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            },
                                            HirEagerComptimeSvarEntry {
                                                name: HirEagerComptimeSvarName::Ident(
                                                    `label`,
                                                ),
                                                data: Inherited,
                                                hir_comptime_symbol: HirTemplateVar::Const(
                                                    HirConstSvar {
                                                        ty: HirType::Svar(
                                                            HirTypeSvar::Type {
                                                                attrs: HirTemplateSvarAttrs {
                                                                    class: Comptime,
                                                                },
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        index: HirConstSvarIndex::Other {
                                                            attrs: HirTemplateSvarAttrs {
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `one_vs_all`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssocType(
                TraitForTypeAssocTypeHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 150,
                            },
                        ),
                    ),
                    hir_decl: TraitForTypeAssocTypeHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 150,
                                },
                            ),
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
                                ItemPath::AssocItem(
                                    AssocItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 150,
                                                },
                                            ),
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
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::AssocFn(
                TraitForTypeAssocFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 151,
                            },
                        ),
                    ),
                    hir_decl: TraitForTypeAssocFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 151,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
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
                                ItemPath::AssocItem(
                                    AssocItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 151,
                                                },
                                            ),
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
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `one_vs_all_result`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `one_vs_all_result`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
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
                                    ItemPath::AssocItem(
                                        AssocItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 151,
                                                    },
                                                ),
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
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 101,
                                                        },
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                            template_arguments: [],
                                                                            always_copyable: true,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 1,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                                            template_arguments: [],
                                                                            always_copyable: true,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 5,
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
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 6,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 101,
                                                        },
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                            template_arguments: [],
                                                                            always_copyable: true,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 1,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                                            template_arguments: [],
                                                                            always_copyable: true,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 5,
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
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::Unit(
                                                    (),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeVariantConstructorCall {
                                                path: TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 100,
                                                        },
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                                            template_arguments: [],
                                                                            always_copyable: true,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 1,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                                            template_arguments: [],
                                                                            always_copyable: true,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
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
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    4..5,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
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
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 9,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 10,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `one_vs_all_result`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
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