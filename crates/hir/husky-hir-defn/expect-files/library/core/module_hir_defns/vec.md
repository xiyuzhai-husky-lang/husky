[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Extern(
                ExternHirDefn {
                    path: TypePath(`core::vec::Vec`, `Extern`),
                    hir_decl: ExternTypeHirDecl {
                        path: TypePath(`core::vec::Vec`, `Extern`),
                        template_parameters: HirTemplateParameters(
                            [
                                HirTemplateParameter {
                                    symbol: HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateSvarAttrs {
                                                class: Comptime,
                                            },
                                            variance: Some(
                                                Covariant,
                                            ),
                                            disambiguator: 0,
                                        },
                                    ),
                                    data: HirTemplateParameterData::Type {
                                        ident: `E`,
                                        traits: [],
                                    },
                                },
                            ],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::vec::Vec`, `Extern`),
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
                                                `E`,
                                            ),
                                            data: Current,
                                            hir_comptime_symbol: HirTemplateVar::Type(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateSvarAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: Some(
                                                        Covariant,
                                                    ),
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
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
                                    ident: `E`,
                                    traits: [],
                                },
                            },
                        ],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeSvarEntry {
                                        name: HirEagerComptimeSvarName::Ident(
                                            `E`,
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Pure,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 23,
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
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                                `E`,
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(core::vec::Vec(0)::push`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(core::vec::Vec(0)::push`, `MethodRitchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: BorrowMut,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Move,
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
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::basic::unit`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::push`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        symbol_modifier: Some(
                                            Owned,
                                        ),
                                        ident: `e`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `E`,
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
                                                `e`,
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
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(core::vec::Vec(0)::first`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(core::vec::Vec(0)::first`, `MethodRitchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: At,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::mem::At`, `Extern`),
                                                template_arguments: [
                                                    HirTemplateArgument::Type(
                                                        HirType::Svar(
                                                            HirTypeSvar::SelfPlace,
                                                        ),
                                                    ),
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
                                                always_copyable: false,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::first`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                                `E`,
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(core::vec::Vec(0)::last`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(core::vec::Vec(0)::last`, `MethodRitchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: At,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::mem::At`, `Extern`),
                                                template_arguments: [
                                                    HirTemplateArgument::Type(
                                                        HirType::Svar(
                                                            HirTypeSvar::SelfPlace,
                                                        ),
                                                    ),
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
                                                always_copyable: false,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::last`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                                `E`,
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodRitchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: BorrowMut,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
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
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                                `E`,
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodRitchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Leash,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                ],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                                `E`,
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodRitchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Leash,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
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
                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                always_copyable: false,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        ident: `start`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `end`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `E`,
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
                                                `start`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `end`,
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
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodRitchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: BorrowMut,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::Ritchie(
                                        HirRitchieType {
                                            ritchie_ty_kind: Fn,
                                            parameters: HirRitchieParameters {
                                                data: [
                                                    HirRitchieParameter::Ordinary(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
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
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
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
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodRitchie(
                                            Fn,
                                        )`),
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
                                        ident: `f`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerComptimeSvarEntry {
                                            name: HirEagerComptimeSvarName::Ident(
                                                `E`,
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
                                                `f`,
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
                    eager_body_with_hir_eager_expr_region: None,
                },
            ),
        ),
    ),
]