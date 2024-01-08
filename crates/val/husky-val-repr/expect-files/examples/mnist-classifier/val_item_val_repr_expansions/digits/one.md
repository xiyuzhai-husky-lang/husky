[
    (
        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
        None,
    ),
    (
        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
        Some(
            ValReprExpansion {
                hir_lazy_variable_val_repr_map: ArenaMap {
                    data: [
                        Some(
                            ValRepr(
                                Id {
                                    value: 23,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 53,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 57,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 61,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 92,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 140,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 142,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 197,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 199,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 208,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 210,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 221,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 223,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 225,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 227,
                                },
                            ),
                        ),
                    ],
                },
                hir_lazy_expr_val_repr_map: [
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 3,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 316,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 5,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 17,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 6,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 7,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 320,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::VecConstructor {
                                    element_ty: LinkageType::Ritchie(
                                        LinkageRitchieType {
                                            parameters: [
                                                LinkageRitchieParameter {
                                                    contract: Pure,
                                                    parameter_ty: PathLeading(
                                                        LinkageTypePathLeading(
                                                            Id {
                                                                value: 3,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ],
                                            return_ty: LinkageType::PathLeading(
                                                LinkageTypePathLeading {
                                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                                    template_arguments: [
                                                        LinkageTemplateArgument::Type(
                                                            LinkageType::PathLeading(
                                                                LinkageTypePathLeading {
                                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                                    template_arguments: [],
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 10,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionFnItem {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 11,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionFnItem {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 43,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 13,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        3.0,
                                    ),
                                    text: "3.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 14,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 24,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 25,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 15,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 18,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 19,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 20,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 29,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 21,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 23,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        6.5,
                                    ),
                                    text: "6.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 24,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Greater,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 32,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 25,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 27,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 28,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 35,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 36,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 29,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 39,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
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
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 30,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionNotSatisfied(
                            ValRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 32,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionNotSatisfied(
                            ValRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 33,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionNotSatisfied(
                            ValRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 34,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 316,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 36,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 37,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 237,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 39,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 40,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 41,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 237,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 43,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 1,
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 44,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 45,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 237,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 47,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 48,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 49,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionFnItem {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 43,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 52,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        4.2,
                                    ),
                                    text: "4.2f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 53,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 64,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 65,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 54,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionFnItem {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 43,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 57,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 68,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 58,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 59,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 69,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 70,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 60,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 71,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 61,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        2.0,
                                    ),
                                    text: "2.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 62,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 64,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Mul,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 74,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 75,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 65,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 67,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 76,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 77,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 68,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        52.0,
                                    ),
                                    text: "52.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 69,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 78,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 79,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 70,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 81,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
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
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 71,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 73,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 85,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 74,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 86,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 75,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 77,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 88,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 78,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 89,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 79,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 80,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 82,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        1.0,
                                    ),
                                    text: "1.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 83,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 94,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 95,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 84,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 87,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 98,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 88,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 90,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 92,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 94,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                12,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 95,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 99,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 100,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 101,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 102,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 103,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 96,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 104,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 97,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 237,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 100,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 101,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 106,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 107,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 102,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 108,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 103,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 109,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 104,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 237,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 106,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 107,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 111,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 112,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 108,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 113,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 109,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 114,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 110,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 115,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 111,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 112,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 110,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 116,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 117,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 113,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 118,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 114,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 117,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 122,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 118,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 123,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 119,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 121,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 125,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 122,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 126,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 123,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 124,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 127,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 124,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 125,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 128,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 129,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 126,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 128,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 132,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 129,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 131,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 134,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 132,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 135,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 133,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 134,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 136,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 137,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 135,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 138,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 136,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 138,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 10,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 138,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 11,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 265,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 140,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 141,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Greater,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 144,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 145,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 142,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 145,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 148,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 146,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 148,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 150,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 149,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 151,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 152,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 152,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 153,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 153,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 154,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 151,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 154,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 155,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 155,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 156,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 156,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 11,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 159,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 11,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 264,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 161,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 11,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 265,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 163,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Div,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 159,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 160,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 164,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 165,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 158,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 161,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 162,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 166,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 163,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 167,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 20,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 237,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 171,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 172,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 167,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 168,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 173,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 169,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 174,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 170,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 175,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 237,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 177,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 178,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 172,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 173,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 179,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 174,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 180,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 175,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 181,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 272,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 237,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 183,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::USize(
                                TermUSizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 184,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 177,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 178,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 185,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 179,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 186,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 180,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 187,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 181,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 188,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 189,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 171,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 176,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 182,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 183,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 190,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 184,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 191,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 22,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 193,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 92,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 188,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 194,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 196,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 191,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 197,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 199,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 193,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 200,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 194,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 201,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 192,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 202,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 192,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 29,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 204,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 206,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 200,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 207,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 209,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinkageTemplateArgument::Type(
                                                LinkageType::PathLeading(
                                                    LinkageTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 365,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 202,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 210,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 203,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 211,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 212,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 204,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 205,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 213,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 201,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 206,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 214,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 201,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 206,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 31,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 208,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 216,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 30,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 265,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 199,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 219,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 208,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 32,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinkageTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 265,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 210,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 221,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 222,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::FunctionGnItem {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinkageTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 211,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 212,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 213,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 223,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 214,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 224,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 22,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 226,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 92,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 217,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 227,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 229,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::ValItem {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValItem(
                            FugitivePath(
                                ItemPathId(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 231,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 35,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 36,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 234,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 37,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 35,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 237,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 37,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        10.0,
                                    ),
                                    text: "10.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 239,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Prefix(
                            Minus,
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 229,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 240,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Geq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 230,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 241,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: LetVariable {
                                stmt: 37,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            TermLiteral::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        20.0,
                                    ),
                                    text: "20.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 243,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Leq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 234,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 244,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 236,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
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
                        arguments: [],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 245,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
                hir_lazy_stmt_val_repr_map: [
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 34,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 1,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 38,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 3,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 66,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 63,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 5,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 80,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 73,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 7,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 81,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 82,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 8,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 130,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 121,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 9,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 146,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 143,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 12,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: ConditionNotSatisfied(
                            ValRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 16,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 46,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 17,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 67,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 72,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 81,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 83,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 21,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 96,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 93,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 23,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 119,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 120,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 131,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 147,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 157,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 164,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 26,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 165,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 166,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 185,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 27,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 189,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 187,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 28,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 218,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 216,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 34,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 231,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 228,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 39,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 235,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 233,
                                    },
                                ),
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 40,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 26,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ],
                            },
                            Branch {
                                condition: None,
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 119,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 165,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 186,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 190,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 215,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 219,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 232,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 236,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 237,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 44,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_val_reprs: [
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssociatedFunctionFn {
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
                                    instantiation: LinkageInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Expr {
                                expr: 7,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 26,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ],
                            },
                            Branch {
                                condition: None,
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 119,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 165,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 186,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 190,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 215,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 219,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 232,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 236,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 237,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: Expansion {
                            parent_val_repr: ValRepr(
                                Id {
                                    value: 13,
                                },
                            ),
                            source: Stmt {
                                stmt: 44,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
            },
        ),
    ),
]