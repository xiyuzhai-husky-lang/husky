```rust
[
    (
        FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
        None,
    ),
    (
        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
        Some(
            KiReprExpansion {
                hir_lazy_variable_ki_repr_map: ArenaMap {
                    data: [
                        Some(
                            KiRepr(
                                Id {
                                    value: 23,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 53,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 57,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 61,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 92,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 140,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 142,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 197,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 199,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 208,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 210,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 221,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 223,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 225,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 227,
                                },
                            ),
                        ),
                    ],
                },
                hir_lazy_expr_ki_repr_map: [
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 2,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 4,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 17,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 5,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 6,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::VecConstructor {
                                    element_ty: LinType::Ritchie(
                                        LinkageRitchieType {
                                            parameters: [
                                                LinkageRitchieParameter {
                                                    contract: Pure,
                                                    parameter_ty: PathLeading(
                                                        LinTypePathLeading(
                                                            Id {
                                                                value: 3,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ],
                                            return_ty: LinType::PathLeading(
                                                LinTypePathLeading {
                                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                                    template_arguments: [
                                                        LinTemplateArgument::Type(
                                                            LinType::PathLeading(
                                                                LinTypePathLeading {
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
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 9,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                        Fn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 10,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                        Fn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 42,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 12,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        3.0,
                                    ),
                                    text: "3.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 13,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 24,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 25,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 14,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 17,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 18,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 19,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 29,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 20,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 22,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        6.5,
                                    ),
                                    text: "6.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 23,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Greater,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 32,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 24,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 26,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 27,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 35,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 36,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 28,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 39,
                                },
                            ),
                        ),
                        opn: KiOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 29,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionNotSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 31,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionNotSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 32,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionNotSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 33,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 35,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 36,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 38,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 39,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 40,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 42,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 1,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 43,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 44,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 46,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 47,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 48,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 17,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                        Fn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 42,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 51,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        4.2,
                                    ),
                                    text: "4.2f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 52,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 64,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 65,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 53,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                        Fn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 42,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 56,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::num::f32(0)::abs`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 68,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 57,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 58,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 69,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 70,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 59,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 71,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 60,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        2.0,
                                    ),
                                    text: "2.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 61,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 63,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Mul,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 74,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 75,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 64,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 66,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 76,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 77,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 67,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        52.0,
                                    ),
                                    text: "52.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 68,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 78,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 79,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 69,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 81,
                                },
                            ),
                        ),
                        opn: KiOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 70,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 17,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 72,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 85,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 73,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::slice::CyclicSlice(0)::end`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 86,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 74,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 17,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 76,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 88,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 77,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::slice::CyclicSlice(0)::start`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 89,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 78,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 79,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 81,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        1.0,
                                    ),
                                    text: "1.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 82,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 94,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 95,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 83,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 86,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 98,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 87,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 89,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 91,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 93,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                12,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 94,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 99,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 100,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 101,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 102,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 103,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 95,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 104,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 96,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 99,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 100,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 106,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 107,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 101,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 108,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 102,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 109,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 103,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 105,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 106,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 111,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 112,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 107,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 113,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 108,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 114,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 109,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::num::f32(0)::abs`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 115,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 110,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 111,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 110,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 116,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 117,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 112,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 118,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 113,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 116,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 122,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 117,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::slice::CyclicSlice(0)::end`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 123,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 118,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 120,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 125,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 121,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::slice::CyclicSlice(0)::start`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 126,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 122,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 124,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 127,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 123,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 124,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 128,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 129,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 125,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 127,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 132,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 128,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 130,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 134,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 131,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::slice::CyclicSlice(0)::start`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 135,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 132,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 133,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 136,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 137,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 134,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 138,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 135,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 138,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 9,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 137,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 10,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 89,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 139,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 140,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Greater,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 144,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 145,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 141,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 144,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 148,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 145,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 147,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 150,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 148,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 150,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 152,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 151,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::num::f32(0)::abs`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 153,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 152,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 153,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 151,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 154,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 155,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 154,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 156,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 155,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 10,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::geom2d::Vector2d(0)::norm`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 158,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 10,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 88,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 160,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 10,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 89,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 162,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Div,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 159,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 160,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 163,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 164,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 158,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 161,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 162,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 165,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 163,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 166,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 170,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 171,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 167,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 168,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 172,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 169,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 173,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 170,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 174,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 176,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 177,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 172,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 173,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 178,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 174,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 179,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 175,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 180,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 182,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 183,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 177,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 178,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 184,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 179,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 185,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 180,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 186,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::num::f32(0)::abs`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 181,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 187,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 188,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 171,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 176,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 182,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 183,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 189,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 184,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 190,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 21,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 192,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 92,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 188,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 193,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 17,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 195,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 191,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 196,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 17,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 198,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 193,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 199,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::slice::CyclicSlice(0)::start`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 194,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 200,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 192,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 201,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 192,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 28,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 203,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 17,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 205,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 200,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 206,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 17,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 208,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
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
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 202,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 209,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `core::slice::CyclicSlice(0)::start`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 203,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 210,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 211,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 204,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 205,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 212,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 201,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 206,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 213,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 201,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 206,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 30,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 208,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 215,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 29,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 89,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 199,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 218,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                            TypeItemKind::MethodRitchie(
                                                RitchieItemKind::Fn,
                                            ),
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 208,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 31,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 89,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 210,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 220,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 221,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFunctionRitchie {
                                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateVariableAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
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
                                    KiRepr(
                                        Id {
                                            value: 211,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 212,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 213,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 222,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 214,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 223,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 21,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 225,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 92,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 217,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 226,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 228,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 230,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 34,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 35,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 233,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 36,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(
                                            `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                                            TypeItemKind::MemoizedField,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 34,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 236,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 36,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        10.0,
                                    ),
                                    text: "10.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 238,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Prefix(
                            Minus,
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 229,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 239,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Geq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 230,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 240,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 36,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: KiOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        20.0,
                                    ),
                                    text: "20.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 242,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: KiOpn::Binary(
                            Comparison(
                                Leq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 234,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 243,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 236,
                                },
                            ),
                        ),
                        opn: KiOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 265,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 244,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
                hir_lazy_stmt_ki_repr_map: [
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 34,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 0,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 38,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 2,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 66,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 63,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 4,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 80,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 73,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 6,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 81,
                                },
                            ),
                        ),
                        opn: KiOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 82,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 7,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 130,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 121,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 8,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 146,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 143,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 11,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: ConditionNotSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 15,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 46,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 16,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 67,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 72,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 81,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 83,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 20,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 96,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 93,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 22,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 119,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 120,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 131,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 147,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 157,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 164,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 25,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 165,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 166,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 185,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 26,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 189,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 187,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 27,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 218,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 216,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 33,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 231,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 228,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 38,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: KiOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 235,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 233,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 39,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 26,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ],
                            },
                            Branch {
                                condition: None,
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 119,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 165,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 186,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 190,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 215,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 219,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 232,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 236,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 237,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 43,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_ki_reprs: [
                    KiRepr {
                        ki_domain_repr: Omni,
                        opn: KiOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocRitchie {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 6,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        ki_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: KiOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 26,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ],
                            },
                            Branch {
                                condition: None,
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 119,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 165,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 186,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 190,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 215,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 219,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 232,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 236,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 237,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                ki_domain_repr: Omni,
                                opn: KiOpn::ValItemLazilyDefined(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 43,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
            },
        ),
    ),
]
```