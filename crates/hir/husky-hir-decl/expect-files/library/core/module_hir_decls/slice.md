```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`core::slice::Slice`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
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
                        region_path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::slice::Slice`, `Extern`),
                                ),
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
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::Extern(
                ExternTypeHirDecl {
                    path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                    template_parameters: HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVariable::Type(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
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
                        region_path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                ),
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
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Current,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                path: AttrItemPath(
                    ItemPathId(
                        Id {
                            value: 254,
                        },
                    ),
                ),
                trais: [
                    HirTrait {
                        trai_path: TraitPath(`core::fmt::Debug`),
                        template_arguments: [],
                    },
                    HirTrait {
                        trai_path: TraitPath(`core::visual::Visualize`),
                        template_arguments: [],
                    },
                ],
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::Decl(
                        ItemPath::Attr(
                            Room32,
                            AttrItemPath(
                                ItemPathId(
                                    Id {
                                        value: 254,
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
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `E`,
                                    ),
                                    data: Inherited,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
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
                    runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                path: TypeImplBlockPath(
                    ItemPathId(
                        Id {
                            value: 138,
                        },
                    ),
                ),
                template_parameters: HirTemplateParameters(
                    [
                        HirTemplateParameter {
                            symbol: HirTemplateVariable::Type(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
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
                        ty_path: TypePath(`core::slice::Slice`, `Extern`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::Variable(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
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
                    region_path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TypeImplBlock(
                                TypeImplBlockPath(
                                    ItemPathId(
                                        Id {
                                            value: 138,
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
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `E`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
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
                    runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeItemPath(`<core::slice::Slice(0)>::len`, `MethodRitchie(
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
                                    value: 20,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::usize`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`<core::slice::Slice(0)>::len`, `MethodRitchie(
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeItemPath(`<core::slice::Slice(0)>::swap`, `MethodRitchie(
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
                                    value: 20,
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
                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            },
                            HirEagerParenateParameter::Simple {
                                pattern_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
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
                        region_path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`<core::slice::Slice(0)>::swap`, `MethodRitchie(
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
                        pattern_arena: Arena {
                            data: [
                                HirEagerPatternEntry {
                                    data: HirEagerPatternData::Ident {
                                        symbol_modifier: None,
                                        ident: `a`,
                                    },
                                    contract: Pure,
                                },
                                HirEagerPatternEntry {
                                    data: HirEagerPatternData::Ident {
                                        symbol_modifier: None,
                                        ident: `b`,
                                    },
                                    contract: Pure,
                                },
                            ],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `a`,
                                        ),
                                        data: HirEagerRuntimeVariableData::ParenateParameter,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `b`,
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
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath(`core::slice::CyclicSlice as core::ops::IntIndex(0)`),
                template_parameters: HirTemplateParameters(
                    [
                        HirTemplateParameter {
                            symbol: HirTemplateVariable::Type(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
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
                trai: HirTrait {
                    trai_path: TraitPath(`core::ops::IntIndex`),
                    template_arguments: [],
                },
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::Variable(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
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
                    region_path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TraitForTypeImplBlock(
                                TraitForTypeImplBlockPath(`core::slice::CyclicSlice as core::ops::IntIndex(0)`),
                            ),
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
                    comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `E`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
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
                    runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
        AssocItemHirDecl::TraitForTypeItem(
            TraitForTypeItemHirDecl::AssocType(
                TraitForTypeAssocTypeHirDecl {
                    path: TraitForTypeItemPath(
                        `<core::slice::CyclicSlice as core::ops::IntIndex(0)>::Output`,
                        TraitItemKind::AssocType,
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    ty: HirType::Variable(
                        HirTypeTemplateVariable::Type {
                            attrs: HirTemplateVariableAttrs {
                                class: Comptime,
                            },
                            variance: None,
                            disambiguator: 0,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TraitForTypeItem(
                                    TraitForTypeItemPath(
                                        `<core::slice::CyclicSlice as core::ops::IntIndex(0)>::Output`,
                                        TraitItemKind::AssocType,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath(
                    ItemPathId(
                        Id {
                            value: 140,
                        },
                    ),
                ),
                template_parameters: HirTemplateParameters(
                    [
                        HirTemplateParameter {
                            symbol: HirTemplateVariable::Type(
                                HirTypeTemplateVariable::Type {
                                    attrs: HirTemplateVariableAttrs {
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
                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                        template_arguments: [
                            HirTemplateArgument::Type(
                                HirType::Variable(
                                    HirTypeTemplateVariable::Type {
                                        attrs: HirTemplateVariableAttrs {
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
                    region_path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TypeImplBlock(
                                TypeImplBlockPath(
                                    ItemPathId(
                                        Id {
                                            value: 140,
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
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [
                                HirEagerComptimeVariableEntry {
                                    name: HirEagerComptimeVariableName::Ident(
                                        `E`,
                                    ),
                                    data: Current,
                                    hir_comptime_symbol: HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
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
                    runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeItemPath(`<core::slice::CyclicSlice(0)>::ilen`, `MethodRitchie(
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
                                    value: 21,
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
                        region_path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`<core::slice::CyclicSlice(0)>::ilen`, `MethodRitchie(
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
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
                                    value: 21,
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
                        region_path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeItemPath(`<core::slice::CyclicSlice(0)>::end`, `MethodRitchie(
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
                                    value: 21,
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
                        region_path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`<core::slice::CyclicSlice(0)>::end`, `MethodRitchie(
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeItemPath(`<core::slice::CyclicSlice(0)>::first`, `MethodRitchie(
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
                                    value: 21,
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
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Variable(
                                                        HirTypeTemplateVariable::Type {
                                                            attrs: HirTemplateVariableAttrs {
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
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`<core::slice::CyclicSlice(0)>::first`, `MethodRitchie(
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeItemPath(`<core::slice::CyclicSlice(0)>::last`, `MethodRitchie(
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
                                    value: 21,
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
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::Variable(
                                                        HirTypeTemplateVariable::Type {
                                                            attrs: HirTemplateVariableAttrs {
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
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`<core::slice::CyclicSlice(0)>::last`, `MethodRitchie(
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerComptimeVariableEntry {
                                        name: HirEagerComptimeVariableName::Ident(
                                            `E`,
                                        ),
                                        data: Inherited,
                                        hir_comptime_symbol: HirTemplateVariable::Type(
                                            HirTypeTemplateVariable::Type {
                                                attrs: HirTemplateVariableAttrs {
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
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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